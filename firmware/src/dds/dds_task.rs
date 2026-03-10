// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use core::cell::RefCell;
use defmt::*;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex as Cs;
use embassy_sync::mutex::Mutex;

use {defmt_rtt as _, panic_probe as _};

use hexa_tune_proto_embedded::command::OperationSub;

use crate::at::{encode_error_response, encode_response, u32_to_ascii_buf};
use crate::channel::*;
use crate::dds::*;
use crate::error::FirmwareError;
use crate::{AT_CH, DDS_CH, RGB_CH};

static OPERATION: Mutex<Cs, RefCell<Operation>> = Mutex::new(RefCell::new(Operation::new()));

#[embassy_executor::task]
pub async fn dds_task(mut ad985x: Ad985x) {
    info!("Starting DDS task");
    loop {
        match DDS_CH.receive().await {
            Msg::OperationCmd { id, sub } => {
                info!("Received OPERATION command in DDS task: {}", id);

                match sub {
                    OperationSub::Prepare => {
                        info!("Preparing DDS operation");

                        let operation = OPERATION.lock().await;
                        *operation.borrow_mut() = Operation::new();
                        {
                            let mut guard = operation.borrow_mut();
                            guard.set_id(id);
                        }
                        drop(operation);

                        info!("DDS operation prepared");
                        let completed =
                            encode_response(b"OPERATION", id, &[b"PREPARE", b"COMPLETED"]);
                        AT_CH.send(Msg::AtCmdResponse(completed.clone())).await;
                        info!("Completed sent for PREPARE command");

                        AT_CH.send(Msg::SetOperationStatus(completed)).await;

                        RGB_CH.send(Msg::RgbMode(RgbState::Prepare)).await;
                    }
                    OperationSub::Generate => {
                        info!("Starting DDS operation");

                        let mut result: Option<FirmwareError> = None;

                        info!("Setting Device Available to false");
                        AT_CH.send(Msg::SetDdsAvailable(false)).await;
                        info!("Set Device Available to false");

                        RGB_CH.send(Msg::RgbMode(RgbState::Generating)).await;

                        let gen_completed =
                            encode_response(b"OPERATION", id, &[b"GENERATE", b"COMPLETED"]);
                        AT_CH
                            .send(Msg::SetOperationStatus(gen_completed.clone()))
                            .await;

                        // Clone steps out of the mutex
                        let step_count;
                        let mut step_ids = [0u32; 64];
                        let mut step_freqs = [0u32; 64];
                        let mut step_times = [0u32; 64];
                        {
                            let operation = OPERATION.lock().await;
                            let guard = operation.borrow();
                            let steps = guard.get_steps();
                            step_count = steps.len();
                            for (i, s) in steps.iter().enumerate() {
                                step_ids[i] = s.id;
                                step_freqs[i] = s.freq;
                                step_times[i] = s.time_ms;
                            }
                        }

                        for i in 0..step_count {
                            let step_id = step_ids[i];
                            let freq = step_freqs[i];
                            let time_ms = step_times[i];

                            // Build status: AT+OPERATION=id#GENERATING#step_id#COMPLETED
                            let mut sid_buf = [0u8; 10];
                            let sid_len = u32_to_ascii_buf(step_id, &mut sid_buf);
                            let status = encode_response(
                                b"OPERATION",
                                id,
                                &[b"GENERATING", &sid_buf[..sid_len], b"COMPLETED"],
                            );
                            AT_CH.send(Msg::SetOperationStatus(status)).await;

                            info!("Setting FREQ to {} over {} ms", freq, time_ms);
                            let err = ad985x.set_freq(freq, time_ms).await;
                            info!("Frequency set complete.");

                            if let Some(err) = err {
                                error!("Error setting FREQ");
                                result = Some(FirmwareError::Hexa(
                                    hexa_tune_proto_embedded::HexaError::InvalidParam,
                                ));
                                let _ = err;
                                break;
                            } else {
                                info!("FREQ set successfully");
                            }
                        }

                        info!("Setting Device Available to true");
                        AT_CH.send(Msg::SetDdsAvailable(true)).await;
                        info!("Set Device Available to true");

                        RGB_CH.send(Msg::RgbMode(RgbState::Idle)).await;

                        if let Some(err) = result {
                            error!("DDS operation failed");
                            AT_CH.send(Msg::Err(id, err)).await;

                            let error_status = encode_error_response(id, &err);
                            AT_CH.send(Msg::SetOperationStatus(error_status)).await;
                        } else {
                            AT_CH.send(Msg::SetOperationStatus(gen_completed)).await;
                        }
                    }
                }
            }
            Msg::FreqSet { id, freq, time_ms } => {
                info!("Received FREQ command in DDS task: {}", id);

                info!("Adding FREQ step to operation");
                let operation = OPERATION.lock().await;

                let step = FreqStep { id, freq, time_ms };
                let add_result = {
                    let mut guard = operation.borrow_mut();
                    guard.add_step(step)
                };
                drop(operation);

                if let Err(e) = add_result {
                    error!("Failed to add step: operation is full");
                    AT_CH.send(Msg::Err(id, e)).await;
                } else {
                    info!("FREQ step added to operation");

                    // Build completed response: AT+FREQ=id#freq#time_ms#COMPLETED
                    let mut freq_buf = [0u8; 10];
                    let freq_len = u32_to_ascii_buf(freq, &mut freq_buf);
                    let mut time_buf = [0u8; 10];
                    let time_len = u32_to_ascii_buf(time_ms, &mut time_buf);
                    let completed = encode_response(
                        b"FREQ",
                        id,
                        &[&freq_buf[..freq_len], &time_buf[..time_len], b"COMPLETED"],
                    );
                    AT_CH.send(Msg::AtCmdResponse(completed)).await;
                    info!("Completed sent for FREQ command");
                }
            }

            _ => break,
        }
    }
}
