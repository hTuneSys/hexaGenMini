// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use core::cell::RefCell;
use defmt::*;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex as Cs;
use embassy_sync::mutex::Mutex;

use heapless::{String, Vec};
use {defmt_rtt as _, panic_probe as _};

use crate::at::*;
use crate::channel::*;
use crate::dds::*;
use crate::error::Error;
use crate::{AT_CH, DDS_CH};

static OPERATION: Mutex<Cs, RefCell<Operation>> = Mutex::new(RefCell::new(Operation::new()));

#[embassy_executor::task]
pub async fn dds_task(mut ad985x: Ad985x) {
    info!("Starting DDS task");
    loop {
        match DDS_CH.receive().await {
            Msg::Operation(at_command) => {
                info!(
                    "Received OPERATION command in DDS task: {}",
                    at_command.id.as_str()
                );
                info!("Parsing OPERATION command parameters");
                if at_command.params.len() != 1 {
                    error!(
                        "OPERATION command requires 1 parameters, got {}",
                        at_command.params.len()
                    );
                    AT_CH.send(Msg::Err(at_command.id, Error::ParamCount)).await;
                    continue;
                }
                info!("Parameters parsed successfully");
                info!("Extracting command_type values");
                let command_type = match at_command.params[0].parse::<String<16>>() {
                    Ok(v) => v,
                    Err(_) => {
                        error!("Invalid command_type value: {}", at_command.id.as_str());
                        AT_CH.send(Msg::Err(at_command.id, Error::ParamValue)).await;
                        continue;
                    }
                };

                match command_type.as_str() {
                    "PREPARE" => {
                        info!("Preparing DDS operation");

                        // Reset the operation by replacing the inner value
                        let operation = OPERATION.lock().await;
                        *operation.borrow_mut() = Operation::new();

                        {
                            let mut guard = operation.borrow_mut();
                            guard.set_id(at_command.id.clone());
                        }
                        drop(operation);

                        info!("DDS operation prepared");
                        AT_CH.send(Msg::Completed(at_command.clone())).await;
                        info!("Completed sent for PREPARE command");

                        let compiled_status = compile_at_completed(at_command.clone());
                        AT_CH.send(Msg::SetOperationStatus(compiled_status)).await;
                    }
                    "GENERATE" => {
                        info!("Starting DDS operation");

                        let mut result: Option<Error> = None;

                        info!("Setting Device Available to false");
                        AT_CH.send(Msg::SetDdsAvailable(false)).await;
                        info!("Set Device Available to false");

                        let compiled_status = compile_at_completed(at_command.clone());
                        AT_CH.send(Msg::SetOperationStatus(compiled_status)).await;

                        let steps = {
                            let operation = OPERATION.lock().await;
                            let guard = operation.borrow();
                            guard.get_steps().clone()
                        };

                        let mut at_command_status = at_command.clone();
                        at_command_status.params.clear();

                        for at_command_step in steps.iter() {
                            let mut params = Vec::<String<16>, 8>::new();
                            let generating_param = String::<16>::try_from("GENERATING").unwrap();
                            params.push(generating_param).ok();

                            at_command_status.params = params;

                            info!("Parsing FREQ command parameters");
                            if at_command_step.params.len() != 2 {
                                error!(
                                    "FREQ command requires 2 parameters, got {}",
                                    at_command_step.params.len()
                                );
                                result = Some(Error::ParamCount);
                                break;
                            }
                            info!("Parameters parsed successfully");

                            info!("Extracting frequency and time_ms values");
                            let freq = match at_command_step.params[0].parse::<u32>() {
                                Ok(v) => v,
                                Err(_) => {
                                    error!(
                                        "Invalid frequency value: {}",
                                        at_command_step.id.as_str()
                                    );
                                    result = Some(Error::ParamValue);
                                    break;
                                }
                            };
                            info!("Frequency value extracted: {}", freq);
                            let time_ms = match at_command_step.params[1].parse::<u32>() {
                                Ok(v) => v,
                                Err(_) => {
                                    error!(
                                        "Invalid time_ms value: {}",
                                        at_command_step.id.as_str()
                                    );
                                    result = Some(Error::ParamValue);
                                    break;
                                }
                            };
                            info!("time_ms value extracted: {}", time_ms);

                            info!(
                                "Setting FREQ to ({}) over {} ms",
                                at_command_step.id.as_str(),
                                time_ms
                            );

                            let step_id_param =
                                String::<16>::try_from(at_command_step.id.as_str()).unwrap();
                            at_command_status.params.push(step_id_param).ok();
                            let compiled_status = compile_at_completed(at_command_status.clone());
                            AT_CH.send(Msg::SetOperationStatus(compiled_status)).await;

                            info!("Starting frequency set...");
                            let err = ad985x.set_freq(freq, time_ms).await;
                            info!("Frequency set complete.");

                            if let Some(err) = err {
                                error!("Error setting FREQ: {:?}", err.code());
                                result = Some(err);
                                break;
                            } else {
                                info!("FREQ set successfully");
                            }
                        }

                        info!("Setting Device Available to true");
                        AT_CH.send(Msg::SetDdsAvailable(true)).await;
                        info!("Set Device Available to true");

                        if let Some(err) = result {
                            error!("DDS operation failed: {:?}", err.code());
                            AT_CH
                                .send(Msg::Err(at_command.id.clone(), err.clone()))
                                .await;

                            let compiled_status = compile_at_error(at_command.id.clone(), err);
                            AT_CH.send(Msg::SetOperationStatus(compiled_status)).await;
                        } else {
                            let compiled_status = compile_at_completed(at_command.clone());
                            AT_CH.send(Msg::SetOperationStatus(compiled_status)).await;
                        }
                    }
                    _ => {
                        error!("Unknown command_type value: {}", at_command.id.as_str());
                        AT_CH.send(Msg::Err(at_command.id, Error::ParamValue)).await;
                        continue;
                    }
                }
            }
            Msg::FreqWithValue(at_command) => {
                info!(
                    "Received FREQ command in DDS task: {}",
                    at_command.id.as_str()
                );

                info!("Adding FREQ command to operation");
                let operation = OPERATION.lock().await;

                let add_result = {
                    let mut guard = operation.borrow_mut();
                    guard.add_step(at_command.clone())
                };
                drop(operation);

                if let Err(e) = add_result {
                    error!("Failed to add step: operation is full");
                    AT_CH.send(Msg::Err(at_command.id, e)).await;
                } else {
                    info!("FREQ command added to operation");
                    AT_CH.send(Msg::Completed(at_command)).await;
                    info!("Completed sent for FREQ command");
                }
            }

            _ => break,
        }
    }
}
