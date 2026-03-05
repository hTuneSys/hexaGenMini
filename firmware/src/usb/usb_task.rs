// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use defmt::{error, info};
use embassy_futures::select::{Either, select};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex as Cs;
use embassy_sync::mutex::Mutex;

use hexa_tune_proto::sysex;
use hexa_tune_proto::usb_midi;

use crate::AT_CH;
use crate::USB_CH;
use crate::channel::*;
use crate::error::FirmwareError;
use crate::usb::{MyMidiClass, MyUsbDevice};

#[embassy_executor::task]
pub async fn dev_task(mut dev: MyUsbDevice<'static>) {
    info!("Starting USB device task");
    dev.run().await;
}

#[embassy_executor::task]
pub async fn usb_io_task(midi: &'static Mutex<Cs, MyMidiClass<'static>>) {
    info!("Starting unified USB IO task");
    loop {
        let read_fut = async {
            let mut buf = [0u8; 64];
            let n = {
                let mut m = midi.lock().await;
                m.read_packet(&mut buf).await.unwrap()
            };
            (n, buf)
        };

        let tx_fut = async { USB_CH.receive().await };

        match select(read_fut, tx_fut).await {
            Either::First((n, buf)) => {
                if n == 0 {
                    continue;
                }
                let data = &buf[..n];
                info!("Received MIDI packet: {:?}", data);

                let num_packets = n / 4;
                if num_packets == 0 {
                    continue;
                }

                // Collect 4-byte packets from raw data
                let mut packets = [[0u8; 4]; 16];
                for i in 0..num_packets.min(16) {
                    packets[i] = [
                        data[i * 4],
                        data[i * 4 + 1],
                        data[i * 4 + 2],
                        data[i * 4 + 3],
                    ];
                }

                // Depacketize USB MIDI → SysEx
                let mut sysex_buf = [0u8; 128];
                let sysex_len = match usb_midi::depacketize(
                    &packets[..num_packets.min(16)],
                    &mut sysex_buf,
                ) {
                    Ok(len) => len,
                    Err(e) => {
                        error!("USB MIDI depacketize error");
                        AT_CH
                            .send(Msg::Err(0, FirmwareError::Proto(e)))
                            .await;
                        continue;
                    }
                };

                // Unframe SysEx → payload
                let payload = match sysex::unframe(&sysex_buf[..sysex_len]) {
                    Ok(p) => p,
                    Err(e) => {
                        error!("SysEx unframe error");
                        AT_CH
                            .send(Msg::Err(0, FirmwareError::Proto(e)))
                            .await;
                        continue;
                    }
                };

                // Convert to UTF-8 string and send to AT channel
                match core::str::from_utf8(payload) {
                    Ok(input) => match MsgString::try_from(input) {
                        Ok(line) => {
                            AT_CH.send(Msg::AtRxLine(line)).await;
                        }
                        Err(_) => {
                            error!("AT payload too long for buffer");
                            AT_CH
                                .send(Msg::Err(
                                    0,
                                    FirmwareError::Proto(
                                        hexa_tune_proto::ProtoError::BufferTooSmall,
                                    ),
                                ))
                                .await;
                        }
                    },
                    Err(_) => {
                        error!("Invalid UTF-8 in payload");
                        AT_CH
                            .send(Msg::Err(
                                0,
                                FirmwareError::Proto(hexa_tune_proto::ProtoError::InvalidUtf8),
                            ))
                            .await;
                    }
                }
            }

            Either::Second(msg) => match msg {
                Msg::UsbTxLine(line) => {
                    let line_bytes = line.as_bytes();
                    let mut sysex_buf = [0u8; 128];
                    match sysex::frame(line_bytes, &mut sysex_buf) {
                        Ok(sysex_len) => {
                            let mut packets = [[0u8; 4]; 32];
                            match usb_midi::packetize(
                                &sysex_buf[..sysex_len],
                                &mut packets,
                            ) {
                                Ok(np) => {
                                    info!("Sending {} MIDI packets", np);
                                    let mut m = midi.lock().await;
                                    for pkt in packets[..np].iter() {
                                        if let Err(e) = m.write_packet(pkt).await {
                                            error!("USB write error: {:?}", e);
                                        }
                                    }
                                }
                                Err(_) => {
                                    error!("USB MIDI packetize error");
                                }
                            }
                        }
                        Err(_) => {
                            error!("UsbTxLine too long to fit into SysEx");
                        }
                    }
                }
                _ => {
                    info!("USB not TX line");
                }
            },
        }
    }
}
