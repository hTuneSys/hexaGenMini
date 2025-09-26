#![no_std]

//! USB (Embassy) for RP2040: CDC-ACM device with a 1 Hz "tick" task.
//!
//! This crate builds the Embassy USB CDC class and exposes ready-to-spawn tasks.
//!
//! Usage from `main` crate (sketch):
//! ```rust
//! use embassy_rp as rp;
//! use embassy_rp::peripherals;
//!
//! // 1) Bind USB IRQ in main (or wherever you own the peripherals):
//! embassy_rp::bind_interrupts!(struct Irqs {
//!     USBCTRL_IRQ => rp::usb::InterruptHandler<peripherals::USB>;
//! });
//!
//! // 2) After rp::init(...):
//! let p = rp::init(Default::default());
//! let driver = rp::usb::Driver::new(p.USB, Irqs);
//!
//! // 3) Build USB device & class inside this crate:
//! let usb::UsbCdc { device, cdc } = usb::init(driver);
//!
//! // 4) Spawn tasks:
//! spawner.spawn(usb::dev_task(device)).unwrap();
//! spawner.spawn(usb::tick_task(cdc)).unwrap();
//! ```

use embassy_time::{Duration, Timer};

use embassy_rp as rp;
use embassy_rp::peripherals;

use embassy_usb::class::cdc_acm::{CdcAcmClass, State as CdcState};
use embassy_usb::{Builder, Config as UsbConfig};

use static_cell::StaticCell;

/// Concrete driver types exported so the tasks can be used by `main`.
pub type Driver<'d> = rp::usb::Driver<'d, peripherals::USB>;
pub type UsbDevice<'d> = embassy_usb::UsbDevice<'d, Driver<'d>>;
pub type Cdc<'d> = CdcAcmClass<'d, Driver<'d>>;

/// Bundle holding the constructed USB device and CDC class instances.
pub struct UsbCdc {
    pub device: UsbDevice<'static>,
    pub cdc: Cdc<'static>,
}

/// Build the Embassy USB CDC-ACM class and device **using a pre-built driver**.
///
/// Pass a `rp::usb::Driver::new(p.USB, Irqs)` created in your `main` crate.
/// This avoids needing to import `Peripheral` traits here and works across SDK versions.
pub fn init(driver: Driver<'static>) -> UsbCdc {
    // USB device configuration (adjust VID/PID/manufacturer/product as needed).
    let mut cfg = UsbConfig::new(0x2E8A, 0x000A); // Raspberry Pi VID + CDC example PID
    cfg.manufacturer = Some("hexaTune");
    cfg.product = Some("hexaGenMini CDC");
    cfg.serial_number = Some("HTS-0001");
    cfg.max_power = 100; // mA
    cfg.max_packet_size_0 = 64; // EP0 size

    // With composite_with_iads (default), set IAD device class triple.
    cfg.device_class = 0xEF;
    cfg.device_sub_class = 0x02;
    cfg.device_protocol = 0x01;

    // Descriptor/control buffers via StaticCell (Rust 2024-safe, no `static mut` refs).
    static CONFIG_DESC_CELL: StaticCell<[u8; 256]> = StaticCell::new();
    static BOS_DESC_CELL: StaticCell<[u8; 256]> = StaticCell::new();
    static MS_OS_DESC_CELL: StaticCell<[u8; 256]> = StaticCell::new();
    static CONTROL_BUF_CELL: StaticCell<[u8; 64]> = StaticCell::new();

    let config_desc = CONFIG_DESC_CELL.init([0; 256]);
    let bos_desc = BOS_DESC_CELL.init([0; 256]);
    let ms_os_desc = MS_OS_DESC_CELL.init([0; 256]);
    let control_buf = CONTROL_BUF_CELL.init([0; 64]);

    // embassy-usb 0.5.x builder.
    let mut builder = Builder::new(driver, cfg, config_desc, bos_desc, ms_os_desc, control_buf);

    // CDC-ACM class and state.
    static CDC_STATE_CELL: StaticCell<CdcState<'static>> = StaticCell::new();
    let cdc_state = CDC_STATE_CELL.init(CdcState::new());
    let cdc = CdcAcmClass::new(&mut builder, cdc_state, 64);

    // Final device must be built after classes are added.
    let dev = builder.build();

    UsbCdc { device: dev, cdc }
}

/// Runs the USB device forever. Must be spawned once.
#[embassy_executor::task]
pub async fn dev_task(mut dev: UsbDevice<'static>) {
    dev.run().await;
}

/// Writes "tick" once per second while the host keeps DTR asserted.
///
/// If you prefer unconditional ticks, replace the body with a plain loop
/// that always writes the packet (and remove `wait_connection`/`dtr` checks).
#[embassy_executor::task]
pub async fn tick_task(mut cdc: Cdc<'static>) {
    loop {
        // Wait until the terminal opens the interface (DTR set).
        let _ = cdc.wait_connection().await;

        // Periodic 1 Hz tick while connected.
        loop {
            if !cdc.dtr() {
                break;
            }
            let _ = cdc.write_packet(b"tick\r\n").await;
            Timer::after(Duration::from_secs(1)).await;
        }
    }
}
