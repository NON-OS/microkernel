// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The identity of a part as the drivers report it, and the parse of the
//! SATA driver's identify reply. Pure, for the host proofs.

/// The SATA driver's identify opcode and the reply it lays out: sectors,
/// sector size, the model and serial lengths, the medium, then the model
/// and serial themselves.
pub const OP_AHCI_IDENTIFY: u16 = 8;
pub const AHCI_IDENTIFY_LEN: usize = 76;
const AHCI_MEDIUM_AT: usize = 14;
const AHCI_MODEL_AT: usize = 16;
const AHCI_SERIAL_AT: usize = 56;
const AHCI_MEDIUM_EMMC: u8 = 1;

#[derive(Clone, Copy, Debug)]
pub struct Identity {
    pub model: [u8; 40],
    pub serial: [u8; 20],
    /// The SATA driver serves an eMMC part, not a SATA disk.
    pub emmc: bool,
}

impl Identity {
    /// The model with the vendor's space padding trimmed.
    pub fn model_str(&self) -> &str {
        trim(&self.model)
    }

    pub fn serial_str(&self) -> &str {
        trim(&self.serial)
    }
}

pub fn trim(field: &[u8]) -> &str {
    core::str::from_utf8(field).unwrap_or("").trim_matches(|c: char| c == ' ' || c == '\0')
}

/// The fields of an AHCI identify body; `None` when it is short.
pub fn parse_ahci_identity(body: &[u8]) -> Option<Identity> {
    if body.len() < AHCI_IDENTIFY_LEN {
        return None;
    }
    let (mut model, mut serial) = ([0u8; 40], [0u8; 20]);
    let model_len = (body[12] as usize).min(40);
    let serial_len = (body[13] as usize).min(20);
    model[..model_len].copy_from_slice(&body[AHCI_MODEL_AT..AHCI_MODEL_AT + model_len]);
    serial[..serial_len].copy_from_slice(&body[AHCI_SERIAL_AT..AHCI_SERIAL_AT + serial_len]);
    Some(Identity { model, serial, emmc: body[AHCI_MEDIUM_AT] == AHCI_MEDIUM_EMMC })
}
