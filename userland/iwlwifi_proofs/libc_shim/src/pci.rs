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

//! The claim and the command register writes, recorded in the same order as
//! the releases, so a proof sees Bus Master go on only after the chip woke
//! and off before any grant is given back.

use std::cell::Cell;

use crate::give_back;

pub const MK_PCI_CFG_COMMAND: u32 = 0x04;
pub const MK_PCI_CMD_MEMORY_SPACE: u16 = 1 << 1;
pub const MK_PCI_CMD_BUS_MASTER: u16 = 1 << 2;
/// The claim epoch the shim's broker hands out.
pub const EPOCH: u64 = 9;

thread_local! {
    static REFUSE: Cell<bool> = const { Cell::new(false) };
}

/// Make the broker refuse command register writes on this thread, or not.
pub fn refuse_config(refuse: bool) {
    REFUSE.with(|r| r.set(refuse));
}

pub fn mk_device_claim(device_id: u64) -> i64 {
    give_back("device_claim", device_id);
    EPOCH as i64
}

/// Recorded as ("pci_command", value); any other register, or another
/// epoch, is a bug in the driver and fails the proof.
pub fn mk_pci_config_write(_device_id: u64, epoch: u64, offset: u32, value: u16) -> i64 {
    assert_eq!(
        (offset, epoch),
        (MK_PCI_CFG_COMMAND, EPOCH),
        "the command register, under the claim"
    );
    give_back("pci_command", u64::from(value));
    if REFUSE.with(|r| r.get()) {
        -1
    } else {
        0
    }
}
