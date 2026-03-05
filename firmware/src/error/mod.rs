// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

pub use hexa_tune_proto::ProtoError;
pub use hexa_tune_proto_embedded::HexaError;

#[derive(Debug, Clone, Copy)]
pub enum FirmwareError {
    Proto(ProtoError),
    Hexa(HexaError),
    OperationStepsFull,
}

impl From<ProtoError> for FirmwareError {
    fn from(e: ProtoError) -> Self {
        Self::Proto(e)
    }
}

impl From<HexaError> for FirmwareError {
    fn from(e: HexaError) -> Self {
        Self::Hexa(e)
    }
}

impl FirmwareError {
    /// Returns a numeric error code (u8) for wire-format encoding.
    pub fn error_code(&self) -> u8 {
        match self {
            FirmwareError::Proto(e) => e.code(),
            FirmwareError::Hexa(e) => match e {
                HexaError::Proto(pe) => pe.code(),
                HexaError::UnknownCommand => 11,
                HexaError::DdsBusy => 12,
                HexaError::NotAQuery => 13,
                HexaError::MissingParam => 14,
                HexaError::InvalidParam => 15,
            },
            FirmwareError::OperationStepsFull => 20,
        }
    }
}

impl defmt::Format for FirmwareError {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FirmwareError({})", self.error_code());
    }
}
