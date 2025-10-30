// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use defmt::{error, info};
use embassy_futures::select::{Either, select};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex as Cs;
use embassy_sync::mutex::Mutex;

use crate::AT_CH;
use crate::USB_CH;
use crate::at::get_empty_id;
use crate::channel::*;
use crate::error::Error;
use crate::sysex::{build_sysex, extract_sysex_payload, sysex_to_usb_midi_packets};
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

                if let Some(payload) = extract_sysex_payload(data) {
                    if let Ok(input) = core::str::from_utf8(&payload) {
                        match heapless::String::<64>::try_from(input) {
                            Ok(line) => {
                                AT_CH.send(Msg::AtRxLine(line)).await;
                            }
                            Err(_) => {
                                error!("Error Code: {}", Error::InvalidDataLength.code());
                                AT_CH
                                    .send(Msg::Err(get_empty_id(), Error::InvalidDataLength))
                                    .await;
                            }
                        }
                    } else {
                        error!("Error Code: {}", Error::InvalidUtf8.code());
                        AT_CH
                            .send(Msg::Err(get_empty_id(), Error::InvalidUtf8))
                            .await;
                    }
                } else {
                    error!("Error Code: {}", Error::InvalidSysEx.code());
                    /*AT_CH
                    .send(Msg::Err(get_empty_id(), Error::InvalidSysEx))
                    .await;*/
                }
            }

            Either::Second(msg) => match msg {
                Msg::UsbTxLine(line) => {
                    if let Some(sysex) = build_sysex::<64>(&line) {
                        let packets = sysex_to_usb_midi_packets::<64>(&sysex);
                        info!("Sending {} MIDI packets", packets.len());

                        let mut m = midi.lock().await;
                        for pkt in packets.iter() {
                            //info!("Sending MIDI packet: {:?}", pkt);
                            if let Err(e) = m.write_packet(pkt).await {
                                error!("USB write error: {:?}", e);
                            }
                        }
                    } else {
                        error!("UsbTxLine too long to fit into SysEx");
                    }
                }
                _ => {
                    info!("USB not TX line");
                }
            },
        }
    }
}
