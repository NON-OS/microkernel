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
//! in their identify page; a person erasing a disk should see the name on
//! its sticker, and the serial's last four characters are the word they type
//! to confirm. The SATA driver and the SD reader answer one identify layout
//! (`laid_out.rs`), the block wire's drivers its IDENTIFY (`wire.rs`), each
//! with a model, a serial and the kind of part. virtio parts give no name.

use super::handle::BlockDevice;
pub use super::identity_parse::Identity;
use super::identity_parse::OP_AHCI_IDENTIFY;
use super::laid_out::laid_out;
use crate::driver::Driver;
use crate::error::BlkError;
use crate::wire::{call, decode_reply, HDR_LEN, STATUS_LEN};

/// The NVMe driver's identify-controller opcode and the payload it lays
/// out: vendor and subsystem ids, then the serial and the model, the
/// firmware revision, the version, the optional admin commands and the
/// namespace count, then MDTS.
const OP_IDENTIFY_CONTROLLER: u16 = 3;
const IDENTIFY_LEN: usize = 88;
const SERIAL_AT: usize = 4;
const MODEL_AT: usize = 24;
pub(super) const MDTS_AT: usize = 82;
/// The SD reader's identify op (capsule_driver_rtsx protocol/ops.rs).
const OP_SD_IDENTIFY: u16 = 6;

impl BlockDevice {
    /// `Ok(None)` for a driver that has no identify page to offer.
    pub fn identity(&self) -> Result<Option<Identity>, BlkError> {
        match self.driver {
            Driver::Nvme => {
                let page = controller_page(self.port)?;
                let (mut model, mut serial) = ([0u8; 40], [0u8; 20]);
                serial.copy_from_slice(&page[SERIAL_AT..SERIAL_AT + 20]);
                model.copy_from_slice(&page[MODEL_AT..MODEL_AT + 40]);
                Ok(Some(Identity { model, serial, emmc: false }))
            }
            Driver::Ahci => Ok(laid_out(Driver::Ahci, OP_AHCI_IDENTIFY, self.port)),
            Driver::Rtsx => Ok(laid_out(Driver::Rtsx, OP_SD_IDENTIFY, self.port)),
            Driver::VirtioBlk => Ok(None),
            _ => Ok(super::wire::identity(self.port, self.unit)),
        }
    }
}

/// The NVMe driver's identify-controller payload, whole.
pub(super) fn controller_page(port: u32) -> Result<[u8; IDENTIFY_LEN], BlkError> {
    let magic = Driver::Nvme.magic();
    let mut rx = [0u8; HDR_LEN + STATUS_LEN + IDENTIFY_LEN];
    let (n, id) = call(port, magic, OP_IDENTIFY_CONTROLLER, 0, &[], &mut rx)?;
    let body = decode_reply(&rx, n, magic, OP_IDENTIFY_CONTROLLER, id)?;
    if body.len() < IDENTIFY_LEN {
        return Err(BlkError::BadLength);
    }
    let mut page = [0u8; IDENTIFY_LEN];
    page.copy_from_slice(&body[..IDENTIFY_LEN]);
    Ok(page)
}
