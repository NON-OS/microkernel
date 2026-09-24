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

//! What the part calls itself. NVMe controllers carry a model and a serial
//! in their identify page, and the driver hands both through; a person
//! erasing a disk should see the name on its sticker, and the last four
//! characters of the serial are the word they type to confirm. SATA and
//! virtio parts give no name through the drivers today and say so.

use super::handle::BlockDevice;
use crate::driver::Driver;
use crate::error::BlkError;
use crate::wire::{call, decode_reply, HDR_LEN, STATUS_LEN};

/// The NVMe driver's identify-controller opcode and the payload it lays
/// out: vendor and subsystem ids, then the serial and the model.
const OP_IDENTIFY_CONTROLLER: u16 = 3;
const IDENTIFY_LEN: usize = 88;
const SERIAL_AT: usize = 4;
const MODEL_AT: usize = 24;

#[derive(Clone, Copy, Debug)]
pub struct Identity {
    pub model: [u8; 40],
    pub serial: [u8; 20],
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

fn trim(field: &[u8]) -> &str {
    core::str::from_utf8(field).unwrap_or("").trim_matches(|c: char| c == ' ' || c == '\0')
}

impl BlockDevice {
    /// `Ok(None)` for a driver that has no identify page to offer.
    pub fn identity(&self) -> Result<Option<Identity>, BlkError> {
        if self.driver != Driver::Nvme {
            return Ok(None);
        }
        let magic = self.driver.magic();
        let mut rx = [0u8; HDR_LEN + STATUS_LEN + IDENTIFY_LEN];
        let (n, id) = call(self.port, magic, OP_IDENTIFY_CONTROLLER, &[], &mut rx)?;
        let body = decode_reply(&rx, n, magic, OP_IDENTIFY_CONTROLLER, id)?;
        if body.len() < MODEL_AT + 40 {
            return Err(BlkError::BadLength);
        }
        let (mut model, mut serial) = ([0u8; 40], [0u8; 20]);
        serial.copy_from_slice(&body[SERIAL_AT..SERIAL_AT + 20]);
        model.copy_from_slice(&body[MODEL_AT..MODEL_AT + 40]);
        Ok(Some(Identity { model, serial }))
    }
}
