// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

#![no_std]
#![no_main]

use defmt::*;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex as Cs;
use embassy_sync::channel::Channel;
use embassy_sync::mutex::Mutex as AsyncMutex;
use {defmt_rtt as _, panic_probe as _};

mod at;
mod channel;
mod dds;
mod error;
mod hexa_config;
mod rgb;
mod sysex;
mod usb;

use crate::channel::*;
pub const CAP: usize = 16;
pub static USB_CH: Channel<Cs, Msg, CAP> = Channel::new();
pub static AT_CH: Channel<Cs, Msg, CAP> = Channel::new();
pub static RGB_CH: Channel<Cs, Msg, CAP> = Channel::new();
pub static DDS_CH: Channel<Cs, Msg, CAP> = Channel::new();

embassy_rp::bind_interrupts!(struct IrqUsb {
    USBCTRL_IRQ => embassy_rp::usb::InterruptHandler<embassy_rp::peripherals::USB>;
});
embassy_rp::bind_interrupts!(struct IrqPio {
    PIO0_IRQ_0 => embassy_rp::pio::InterruptHandler<embassy_rp::peripherals::PIO0>;
});

#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) {
    //Initialize the RP2040
    let p = embassy_rp::init(Default::default());
    info!("Starting hexaGenMini firmware");

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

    spawner.spawn(at::at_task(dispatcher, spawner)).unwrap();
    spawner.spawn(rgb::rgb_task(rgb_led)).unwrap();
    spawner.spawn(usb::dev_task(device)).unwrap();
    spawner.spawn(usb::usb_io_task(midi_mutex)).unwrap();
    spawner.spawn(dds::dds_task(ad9850)).unwrap();
    spawner.spawn(main_loop_task(led)).unwrap();
}

#[embassy_executor::task]
pub async fn main_loop_task(mut led: embassy_rp::gpio::Output<'static>) {
    //Dummy task to blink the LED
    loop {
        info!("Blink!");

        if hexa_config::is_dds_available() {
            led.set_high();
            embassy_time::Timer::after_secs(5).await;
            led.set_low();
            embassy_time::Timer::after_secs(5).await;
        } else {
            for _ in 0..50 {
                led.set_high();
                embassy_time::Timer::after_millis(100).await;
                led.set_low();
                embassy_time::Timer::after_millis(100).await;
            }
        }
    }
}
