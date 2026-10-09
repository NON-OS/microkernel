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
//! characters of the serial are the word they type to confirm. The SATA
//! driver answers its own identify op with the ATA model and serial, and
//! says whether the part it serves is a SATA disk or an eMMC one. virtio
//! parts give no name and say so.

use super::handle::BlockDevice;
pub use super::identity_parse::Identity;
use super::identity_parse::{parse_ahci_identity, AHCI_IDENTIFY_LEN, OP_AHCI_IDENTIFY};
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
            Driver::Ahci => Ok(ahci_identity(self.port)),
            Driver::VirtioBlk => Ok(None),
        }
    }
}

/// The NVMe driver's identify-controller payload, whole.
pub(super) fn controller_page(port: u32) -> Result<[u8; IDENTIFY_LEN], BlkError> {
    let magic = Driver::Nvme.magic();
    let mut rx = [0u8; HDR_LEN + STATUS_LEN + IDENTIFY_LEN];
    let (n, id) = call(port, magic, OP_IDENTIFY_CONTROLLER, &[], &mut rx)?;
    let body = decode_reply(&rx, n, magic, OP_IDENTIFY_CONTROLLER, id)?;
    if body.len() < IDENTIFY_LEN {
        return Err(BlkError::BadLength);
    }
    let mut page = [0u8; IDENTIFY_LEN];
    page.copy_from_slice(&body[..IDENTIFY_LEN]);
    Ok(page)
}

/// The SATA driver's identify reply, or `None` from a driver built before
/// it had one: the disk is still listed, by its bus and size alone.
fn ahci_identity(port: u32) -> Option<Identity> {
    let magic = Driver::Ahci.magic();
    let mut rx = [0u8; HDR_LEN + STATUS_LEN + AHCI_IDENTIFY_LEN];
    let (n, id) = call(port, magic, OP_AHCI_IDENTIFY, &[], &mut rx).ok()?;
    let body = decode_reply(&rx, n, magic, OP_AHCI_IDENTIFY, id).ok()?;
    parse_ahci_identity(body)
}
