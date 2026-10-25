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

//! Claim, the PCI command write and the port grants, answered as the test
//! scripts them and recorded with every other broker call, in order.

use crate::script::{answer, note};

pub const MK_PCI_CFG_COMMAND: u32 = 0x04;
pub const MK_PCI_CMD_BUS_MASTER: u16 = 1 << 2;
pub const MK_PCI_CMD_INTX_DISABLE: u16 = 1 << 10;

#[repr(C)]
pub struct PioGrantOut {
    pub port_base: u16,
    pub port_count: u16,
    pub _pad: u32,
    pub grant_id: u64,
}

pub fn mk_device_claim(device_id: u64) -> i64 {
    note("claim", device_id);
    answer().claim
}
pub fn mk_device_release(device_id: u64) -> i64 {
    note("device_release", device_id);
    0
}
pub fn mk_pci_config_write(_d: u64, epoch: u64, offset: u32, value: u16) -> i64 {
    assert_eq!((epoch, offset), (7, MK_PCI_CFG_COMMAND), "the command register, under the claim");
    note("pci_command", value as u64);
    answer().command
}
pub fn mk_pio_grant(_d: u64, epoch: u64, bar: u8, _f: u32, out: *mut PioGrantOut) -> i64 {
    assert_eq!(epoch, 7, "a grant under the claim's epoch");
    note("pio_grant", bar as u64);
    let count = answer().ports.get(bar as usize).copied().unwrap_or(0);
    if count == 0 {
        return -1;
    }
    let grant =
        PioGrantOut { port_base: 0xC000, port_count: count, _pad: 0, grant_id: 50 + bar as u64 };
    // SAFETY: the caller hands a writable PioGrantOut, as the real call requires.
    unsafe { *out = grant };
    0
}
pub fn mk_pio_release(grant: u64) -> i64 {
    note("pio_release", grant);
    0
}
