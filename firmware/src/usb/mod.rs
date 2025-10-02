// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use core::fmt::Write;
use defmt::info;
use embassy_usb::class::midi::MidiClass;
use embassy_usb::{Builder, Config};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

pub type MyDriver<'d> = embassy_rp::usb::Driver<'d, embassy_rp::peripherals::USB>;
pub type MyUsbDevice<'d> = embassy_usb::UsbDevice<'d, MyDriver<'d>>;
pub type MyMidiClass<'d> = embassy_usb::class::midi::MidiClass<'d, MyDriver<'d>>;

pub struct UsbMidi {
    pub device: MyUsbDevice<'static>,
    pub midi: MyMidiClass<'static>,
}

pub fn init(driver: MyDriver<'static>) -> UsbMidi {
    let mut cfg = Config::new(0x2E8A, 0x0010);
    cfg.manufacturer = Some("hexaTune");
    cfg.product = Some("hexaGenMini MIDI");
    cfg.serial_number = Some("HTS-0001");
    cfg.max_power = 100;
    cfg.max_packet_size_0 = 64;
    //cfg.device_class = 0x00;
    //cfg.device_sub_class = 0x00;
    //cfg.device_protocol = 0x00;

    static CONFIG_DESC: StaticCell<[u8; 256]> = StaticCell::new();
    static BOS_DESC: StaticCell<[u8; 256]> = StaticCell::new();
    static MS_OS_DESC: StaticCell<[u8; 256]> = StaticCell::new();
    static CONTROL_BUF: StaticCell<[u8; 64]> = StaticCell::new();

    let config_desc = CONFIG_DESC.init([0; 256]);
    let bos_desc = BOS_DESC.init([0; 256]);
    let ms_os_desc = MS_OS_DESC.init([0; 256]);
    let control_buf = CONTROL_BUF.init([0; 64]);

    let mut builder = Builder::new(driver, cfg, config_desc, bos_desc, ms_os_desc, control_buf);

    let midi = MidiClass::new(&mut builder, 1, 1, 64);

    let dev = builder.build();
    UsbMidi { device: dev, midi }
}

#[embassy_executor::task]
pub async fn dev_task(mut dev: MyUsbDevice<'static>) {
    dev.run().await;
}

#[embassy_executor::task]
pub async fn usb_io_task(
    mut midi: MyMidiClass<'static>,
    dispatcher: &'static crate::at::AtDispatcher,
) {
    let mut buf = [0; 64];
    loop {
        let n = midi.read_packet(&mut buf).await.unwrap();
        let data = &buf[..n];
        info!("Received MIDI packet: {:?}", data);
        if let Some(payload) = crate::sysex::extract_sysex_payload(data) {
            if let Ok(input) = core::str::from_utf8(&payload) {
                let response = dispatcher.dispatch(input);
                info!("Received AT command: {}", input);
                info!("AT response: {}", response.as_str());
                if let Some(sysex) = crate::sysex::build_sysex::<64>(&response) {
                    let packets = crate::sysex::sysex_to_usb_midi_packets::<64>(&sysex);
                    for pkt in packets.iter() {
                        match midi.write_packet(pkt).await {
                            Ok(_) => {
                                info!("Sent AT response via SysEx");
                            }
                            Err(e) => {
                                info!("Failed to send MIDI packet: {:?}", e);
                            }
                        }
                    }
                } else {
                    info!("Response too long to fit in SysEx");
                }
            } else {
                let error = crate::error::Error::InvalidUtf8.description();
                let encoded = crate::b64::encode(error);
                let mut response = heapless::String::<64>::new();
                write!(response, "AT+ERROR={}", encoded).unwrap();
                if let Some(sysex) = crate::sysex::build_sysex::<64>(&response) {
                    let packets = crate::sysex::sysex_to_usb_midi_packets::<64>(&sysex);
                    for pkt in packets.iter() {
                        match midi.write_packet(pkt).await {
                            Ok(_) => {
                                info!("Sent AT response via SysEx");
                            }
                            Err(e) => {
                                info!("Failed to send MIDI packet: {:?}", e);
                            }
                        }
                    }
                } else {
                    info!("Error response too long to fit in SysEx");
                }
            }
        } else {
            let error = crate::error::Error::InvalidSysEx.description();
            let encoded = crate::b64::encode(error);
            let mut response = heapless::String::<64>::new();
            write!(response, "AT+ERROR={}", encoded).unwrap();
            if let Some(sysex) = crate::sysex::build_sysex::<64>(&response) {
                let packets = crate::sysex::sysex_to_usb_midi_packets::<64>(&sysex);
                for pkt in packets.iter() {
                    match midi.write_packet(pkt).await {
                        Ok(_) => {
                            info!("Sent AT response via SysEx");
                        }
                        Err(e) => {
                            info!("Failed to send MIDI packet: {:?}", e);
                        }
                    }
                }
            } else {
                info!("Error response too long to fit in SysEx");
            }
        }
    }
}
