// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

#![no_std]
#![no_main]

use defmt::info;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex as Cs;
use embassy_sync::mutex::Mutex as AsyncMutex;
use {defmt_rtt as _, panic_probe as _};

mod at;
mod b64;
mod channel;
mod error;
mod hexa_config;
mod rgb;
mod sysex;
mod usb;

static CHANNEL_MANAGER: static_cell::StaticCell<channel::ChannelManager<{ channel::CAP }, 8>> =
    static_cell::StaticCell::new();

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

    //Create the channel manager
    info!("Initializing channel manager");
    let mut mgr = channel::ChannelManager::<{ channel::CAP }, 8>::new();
    let usb_rx = mgr.register(channel::ModuleId::Usb, &channel::USB_CH);
    let at_rx = mgr.register(channel::ModuleId::At, &channel::AT_CH);
    let rgb_rx = mgr.register(channel::ModuleId::Rgb, &channel::RGB_CH);
    let _dds_rx = mgr.register(channel::ModuleId::Dds, &channel::DDS_CH);
    let mgr_ref: &'static channel::ChannelManager<{ channel::CAP }, 8> = CHANNEL_MANAGER.init(mgr);
    let at_tx = mgr_ref.tx(channel::ModuleId::At).unwrap();
    let usb_tx = mgr_ref.tx(channel::ModuleId::Usb).unwrap();
    let rgb_tx = mgr_ref.tx(channel::ModuleId::Rgb).unwrap();
    //let dds_tx = mgr_ref.tx(channel::ModuleId::Dds).unwrap();

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

    //Spawn tasks
    info!("Spawning tasks");
    spawner
        .spawn(at::at_task(
            dispatcher, spawner, at_rx, at_tx, usb_tx, rgb_tx,
        ))
        .unwrap();
    spawner.spawn(usb::dev_task(device)).unwrap();
    // midi_mutex now matches the expected type for usb_io_task (AsyncMutex<Cs, ...>)
    spawner
        .spawn(usb::usb_io_task(midi_mutex, usb_rx, at_tx))
        .unwrap();
    spawner
        .spawn(rgb::rgb_task(rgb_led, rgb_rx, at_tx))
        .unwrap();

    //Dummy task to blink the LED
    let mut led = embassy_rp::gpio::Output::new(p.PIN_25, embassy_rp::gpio::Level::Low);
    loop {
        info!("Blink!");
        led.set_high();
        embassy_time::Timer::after_secs(5).await;
        led.set_low();
        embassy_time::Timer::after_secs(5).await;
    }
}
