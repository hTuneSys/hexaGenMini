#![no_std]
#![no_main]

use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

use embassy_rp as rp;
use rp2040_boot2 as _;

/// Minimal timestamp for `defmt` (optional).
#[unsafe(no_mangle)]
fn _defmt_timestamp() -> u64 {
    0
}

// Bind the USB interrupt (executor runs in thread mode; no SWI/TIMER IRQs here).
embassy_rp::bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => embassy_rp::usb::InterruptHandler<embassy_rp::peripherals::USB>;
});

/// Embassy entry point: set up USB CDC and tick once per second over USB.
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // Initialize RP2040 (time driver is installed via "time-driver" feature).
    let p = rp::init(Default::default());

    // Build the Embassy RP USB driver in main.
    let driver = rp::usb::Driver::new(p.USB, Irqs);

    // Build USB device + CDC class via the usb crate.
    let usb::UsbCdc { device, cdc } = usb::init(driver);

    // Spawn the device runner and the 1 Hz ticker tasks.
    spawner.spawn(usb::dev_task(device)).unwrap();
    spawner.spawn(usb::tick_task(cdc)).unwrap();

    // Park the main task.
    loop {
        Timer::after(Duration::from_secs(3600)).await;
    }
}
