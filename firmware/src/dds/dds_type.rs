// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use heapless::{String, Vec};

use crate::at::AtCommand;
use crate::error::Error;

pub type OperationId = String<16>;

pub struct Operation {
    id: OperationId,
    steps: Vec<AtCommand, 64>,
}

impl Operation {
    pub const fn new() -> Self {
        Self {
            id: String::new(),
            steps: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn get_id(&self) -> &OperationId {
        &self.id
    }

    pub fn set_id(&mut self, id: OperationId) {
        self.id = id;
    }

    pub fn get_steps(&self) -> &heapless::Vec<AtCommand, 64> {
        &self.steps
    }

    pub fn add_step(&mut self, step: AtCommand) -> Result<(), Error> {
        self.steps.push(step).map_err(|_| Error::OperationStepsFull)
    }
}
