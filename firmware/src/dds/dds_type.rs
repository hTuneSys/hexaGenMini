// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use heapless::Vec;

use crate::error::FirmwareError;

pub struct FreqStep {
    pub id: u32,
    pub freq: u32,
    pub time_ms: u32,
}

pub struct Operation {
    id: u32,
    steps: Vec<FreqStep, 64>,
}

impl Operation {
    pub const fn new() -> Self {
        Self {
            id: 0,
            steps: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    pub fn get_steps(&self) -> &Vec<FreqStep, 64> {
        &self.steps
    }

    pub fn add_step(&mut self, step: FreqStep) -> Result<(), FirmwareError> {
        self.steps
            .push(step)
            .map_err(|_| FirmwareError::OperationStepsFull)
    }
}
