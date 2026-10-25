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

//! The claim, its release, and the device's PCI config space.

use crate::calls::Call;
use crate::log::note;
use crate::MK_PCI_CFG_COMMAND;

/// The epoch every claim is given.
const EPOCH: i64 = 3;
const EBUSY: i64 = -16;
const EINVAL: i64 = -22;

pub fn mk_device_claim(_device_id: u64) -> i64 {
    if note(Call::Claim) {
        EPOCH
    } else {
        EBUSY
    }
}

/// The release, which frees the device's DMA grants, as the kernel's does
/// once Bus Master reads back off.
pub fn mk_device_release(device_id: u64) -> i64 {
    note(Call::Release);
    crate::dma_back::free_device(device_id);
    0
}

/// A Command write, logged whether or not it is refused.
pub fn mk_pci_config_write(_device_id: u64, epoch: u64, offset: u32, value: u16) -> i64 {
    if !note(Call::Command(value)) {
        return EBUSY;
    }
    if offset != MK_PCI_CFG_COMMAND || epoch != EPOCH as u64 {
        return EINVAL;
    }
    0
}

/// A config read, answered from the config the test set (config.rs), or 0:
/// the SDHCI Slot Information byte's one slot, its registers in BAR0.
pub fn mk_pci_config_read(_device_id: u64, _epoch: u64, offset: u32, width: u32) -> i64 {
    crate::config::read(offset, width)
}
