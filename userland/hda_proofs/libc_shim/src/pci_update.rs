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

//! The masked form of the command write, as the broker takes it
//! (src/hardware/broker/pci/change.rs): the bits the mask names take the
//! value given, set or clear, and every other bit stays as it is. A value
//! bit the mask does not name is refused.

use std::cell::Cell;

use crate::broker::record;
use crate::pci::{COMMAND, MK_PCI_CFG_COMMAND, REFUSED};

pub fn mk_pci_config_update(
    _device_id: u64,
    _epoch: u64,
    offset: u32,
    mask: u16,
    value: u16,
) -> i64 {
    record("pci_config_update", value as u64);
    if offset != MK_PCI_CFG_COMMAND || value & !mask != 0 {
        return -22;
    }
    if REFUSED.with(Cell::get) {
        return -1;
    }
    COMMAND.with(|c| c.set((c.get() & !mask) | value));
    0
}
