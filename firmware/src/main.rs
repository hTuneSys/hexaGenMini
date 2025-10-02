// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio;
use embassy_rp::peripherals::USB;
use embassy_rp::usb::Driver;
use embassy_rp::usb::InterruptHandler;
use embassy_time::Timer;
use gpio::{Level, Output};
use {defmt_rtt as _, panic_probe as _};

mod at;
mod b64;
mod error;
mod sysex;
mod usb;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    info!("Starting hexaGenMini firmware");
    let driver = Driver::new(p.USB, Irqs);
    let usb::UsbMidi { device, midi } = usb::init(driver);
    static DISPATCHER: static_cell::StaticCell<at::AtDispatcher> = static_cell::StaticCell::new();
    let dispatcher = DISPATCHER.init(at::AtDispatcher::new());
    spawner.spawn(usb::dev_task(device)).unwrap();
    spawner.spawn(usb::usb_io_task(midi, dispatcher)).unwrap();

    let mut led = Output::new(p.PIN_25, Level::Low);
    loop {
        led.set_high();
        Timer::after_secs(1).await;
        led.set_low();
        Timer::after_secs(1).await;
    }
}
