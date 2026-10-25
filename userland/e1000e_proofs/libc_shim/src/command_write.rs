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

//! The two forms of the Command write, as the broker takes them
//! (src/hardware/broker/pci/command_bits.rs and change.rs). The plain form
//! ORs its bits onto the register, so it turns bits on and never off. The
//! masked form names the bits it changes, and each takes the value given,
//! set or clear. Either refuses a value carrying a bit outside the
//! allowlist, and the masked one a value bit its mask does not name.

use crate::command::{store, MK_PCI_CFG_COMMAND, WRITABLE};

pub fn mk_pci_config_write(_device_id: u64, _epoch: u64, offset: u32, value: u16) -> i64 {
    if offset != MK_PCI_CFG_COMMAND || value & !WRITABLE != 0 {
        return -22;
    }
    store(value, value)
}

pub fn mk_pci_config_update(
    _device_id: u64,
    _epoch: u64,
    offset: u32,
    mask: u16,
    value: u16,
) -> i64 {
    if offset != MK_PCI_CFG_COMMAND || mask & !WRITABLE != 0 || value & !mask != 0 {
        return -22;
    }
    store(value, mask)
}
