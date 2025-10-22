// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

#![no_std]
#![no_main]

use defmt::*;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex as Cs;
use embassy_sync::mutex::Mutex as AsyncMutex;
use heapless::{String, Vec};
use {defmt_rtt as _, panic_probe as _};

mod at;
mod channel;
mod dds;
mod error;
mod hexa_config;
mod rgb;
mod sysex;
mod usb;

use crate::channel::CAP;

static mut CORE1_STACK: embassy_rp::multicore::Stack<4096> = embassy_rp::multicore::Stack::new();
static EXECUTOR0: static_cell::StaticCell<embassy_executor::Executor> =
    static_cell::StaticCell::new();
static EXECUTOR1: static_cell::StaticCell<embassy_executor::Executor> =
    static_cell::StaticCell::new();
static CHANNEL_MANAGER: static_cell::StaticCell<channel::ChannelManager<{ channel::CAP }, 8>> =
    static_cell::StaticCell::new();

embassy_rp::bind_interrupts!(struct IrqUsb {
    USBCTRL_IRQ => embassy_rp::usb::InterruptHandler<embassy_rp::peripherals::USB>;
});
embassy_rp::bind_interrupts!(struct IrqPio {
    PIO0_IRQ_0 => embassy_rp::pio::InterruptHandler<embassy_rp::peripherals::PIO0>;
});

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    //Initialize the RP2040
    let p = embassy_rp::init(Default::default());
    info!("Starting hexaGenMini firmware");

    //Create the channel manager
    info!("Initializing channel manager");
    let mut mgr = channel::ChannelManager::<{ channel::CAP }, 8>::new();
    let usb_rx = mgr.register(channel::ModuleId::Usb, &channel::USB_CH);
    let at_rx = mgr.register(channel::ModuleId::At, &channel::AT_CH);
    let rgb_rx = mgr.register(channel::ModuleId::Rgb, &channel::RGB_CH);
    let dds_rx = mgr.register(channel::ModuleId::Dds, &channel::DDS_CH);
    let mgr_ref: &'static channel::ChannelManager<{ channel::CAP }, 8> = CHANNEL_MANAGER.init(mgr);
    let at_tx = mgr_ref.tx(channel::ModuleId::At).unwrap();
    let usb_tx = mgr_ref.tx(channel::ModuleId::Usb).unwrap();
    let rgb_tx = mgr_ref.tx(channel::ModuleId::Rgb).unwrap();
    let dds_tx = mgr_ref.tx(channel::ModuleId::Dds).unwrap();

    //USB module
    info!("Initializing USB");
    let driver = embassy_rp::usb::Driver::new(p.USB, IrqUsb);
    let usb::UsbMidi { device, midi } = usb::init(driver);

    static MIDI_CELL: static_cell::StaticCell<AsyncMutex<Cs, usb::MyMidiClass<'static>>> =
        static_cell::StaticCell::new();
    let midi_mutex: &'static AsyncMutex<Cs, usb::MyMidiClass<'static>> =
        MIDI_CELL.init(AsyncMutex::new(midi));

    //AT module
    info!("Initializing AT command dispatcher");
    static DISPATCHER: static_cell::StaticCell<at::AtDispatcher> = static_cell::StaticCell::new();
    let dispatcher = DISPATCHER.init(at::AtDispatcher::new());

    //Led module
    info!("Initializing RGB LED");
    let embassy_rp::pio::Pio {
        mut common, sm0, ..
    } = embassy_rp::pio::Pio::new(p.PIO0, IrqPio);
    let program = embassy_rp::pio_programs::ws2812::PioWs2812Program::new(&mut common);
    let ws2812 = embassy_rp::pio_programs::ws2812::PioWs2812::new(
        &mut common,
        sm0,
        p.DMA_CH0,
        p.PIN_23,
        &program,
    );
    let rgb_led = rgb::RgbLed::new(ws2812);

    //DDS module
    let ad9850 = dds::Ad985x::new(
        embassy_rp::gpio::Output::new(p.PIN_2, embassy_rp::gpio::Level::Low),
        embassy_rp::gpio::Output::new(p.PIN_3, embassy_rp::gpio::Level::Low),
        embassy_rp::gpio::Output::new(p.PIN_4, embassy_rp::gpio::Level::Low),
        embassy_rp::gpio::Output::new(p.PIN_5, embassy_rp::gpio::Level::Low),
        125_000_000,
        0,
    );

    //Dummy Led
    let led = embassy_rp::gpio::Output::new(p.PIN_25, embassy_rp::gpio::Level::Low);

    embassy_rp::multicore::spawn_core1(
        p.CORE1,
        unsafe { &mut *core::ptr::addr_of_mut!(CORE1_STACK) },
        move || {
            let executor1 = EXECUTOR1.init(embassy_executor::Executor::new());
            executor1.run(|spawner| {
                unwrap!(spawner.spawn(dds::dds_task(ad9850, dds_rx, at_tx)));
            });
        },
    );

    //Core0 Task Spawner
    let executor0 = EXECUTOR0.init(embassy_executor::Executor::new());
    executor0.run(|spawner| {
        //RGB task
        unwrap!(spawner.spawn(rgb::rgb_task(rgb_led, rgb_rx, at_tx)));
        //AT tasks
        unwrap!(spawner.spawn(at::at_task(
            dispatcher, spawner, at_rx, at_tx, usb_tx, rgb_tx, dds_tx,
        )));
        //USB Device task
        unwrap!(spawner.spawn(usb::dev_task(device)));
        //USB IO task
        unwrap!(spawner.spawn(usb::usb_io_task(midi_mutex, usb_rx, at_tx)));
        //Main loop task
        unwrap!(spawner.spawn(main_loop_task(at_tx, led)));
    });
}

#[embassy_executor::task]
pub async fn main_loop_task(
    at_tx: embassy_sync::channel::Sender<'static, Cs, channel::Msg, CAP>,
    mut led: embassy_rp::gpio::Output<'static>,
) {
    //Dummy task to blink the LED
    loop {
        info!("Blink!");
        let mut name = String::<16>::new();
        name.push_str("STATUS").unwrap();

        let mut params = Vec::<String<16>, 8>::new();
        let mut status_param = String::<16>::try_from("AVAILABLE").unwrap();

        if hexa_config::is_dds_available() {
            led.set_high();
            embassy_time::Timer::after_secs(5).await;
            led.set_low();
            embassy_time::Timer::after_secs(5).await;
        } else {
            status_param = String::<16>::try_from("GENERATING").unwrap();
            for _ in 0..5 {
                led.set_high();
                embassy_time::Timer::after_secs(1).await;
                led.set_low();
                embassy_time::Timer::after_secs(1).await;
            }
        }
        params.push(status_param).ok();
        let at_command = at::AtCommand {
            id: at::get_empty_id(),
            name,
            params,
            is_query: false,
        };
        at_tx.send(channel::Msg::AtCmdOutput(at_command)).await;
    }
}
