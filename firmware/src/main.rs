#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use rp2040_hal as hal;

use hal::{clocks::init_clocks_and_plls, pac, sio::Sio, watchdog::Watchdog};

use usb_device::class_prelude::UsbBusAllocator;
use usb_device::device::UsbDeviceState;
use usb_device::prelude::{UsbDeviceBuilder, UsbVidPid};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

#[unsafe(no_mangle)]
fn _defmt_timestamp() -> u64 {
    0
}

// Boot2 must be placed at .boot2 so ROM finds it at 0x10000000
#[unsafe(link_section = ".boot2")]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

#[entry]
fn main() -> ! {
    // --- Peripherals ---
    let mut pac = pac::Peripherals::take().unwrap();
    let _core = pac::CorePeripherals::take().unwrap();

    // --- Clocks ---
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let clocks = init_clocks_and_plls(
        12_000_000u32, // Pico/Pico W external crystal
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // --- GPIO ---
    let sio = Sio::new(pac.SIO);
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led = pins.gpio0.into_push_pull_output();

    // --- Timer ---
    let mut timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    // --- USB CDC ---
    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true, // force_vbus_detect: fine for bus-powered
        &mut pac.RESETS,
    ));
    let mut serial = SerialPort::new(&usb_bus);
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x2E8A, 0x000A))
        .device_class(USB_CLASS_CDC)
        .build();

    // --- Main loop ---
    let mut counter: u32 = 0;
    let mut last_state = UsbDeviceState::Default;
    let mut ms_acc: u32 = 0;
    let mut led_on = false;

    loop {
        // Poll the USB stack every millisecond
        if usb_dev.poll(&mut [&mut serial]) {
            // Optional: echo incoming bytes
            let mut buf = [0u8; 64];
            if let Ok(n) = serial.read(&mut buf)
                && n > 0
            {
                let _ = serial.write(b"echo: ");
                let _ = serial.write(&buf[..n]);
            }
        }

        // When first becoming Configured, flash LED quickly (visual cue)
        let state = usb_dev.state();
        if state != last_state && state == UsbDeviceState::Configured {
            for _ in 0..3 {
                let _ = led.set_high();
                timer.delay_ms(40);
                let _ = led.set_low();
                timer.delay_ms(40);
            }
        }
        last_state = state;

        // Send logs only if host opened the port (DTR set)
        if state == UsbDeviceState::Configured && serial.dtr() {
            counter = counter.wrapping_add(1);
            let _ = serial.write(b"tick ");
            let mut num = itoa::Buffer::new();
            let s = num.format(counter);
            let _ = serial.write(s.as_bytes());
            let _ = serial.write(b"\r\n");
        }

        // 1 ms tick to avoid long blocking delays
        timer.delay_ms(1);
        ms_acc += 1;

        // 500 ms blink using the 1 ms tick
        if ms_acc >= 500 {
            if led_on {
                let _ = led.set_low();
            } else {
                let _ = led.set_high();
            }
            led_on = !led_on;
            ms_acc = 0;
        }
    }
}
