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

//! Bus mastering off at claim (L34). Firmware may leave it on, and until its
//! driver resets it a device may still hold a ring firmware programmed: on a
//! machine without an IOMMU its first DMA could reach any page. The driver
//! turns it on through the config-write allowlist once its own rings exist,
//! and release turns it off again.

use crate::drivers::pci::config::ConfigSpace;

pub(super) fn bus_master_off(device_id: u64) {
    let Some(handle) = crate::hardware::broker::pci_index::lookup(device_id) else {
        return;
    };
    let cfg = ConfigSpace::new(handle.address);
    if !matches!(cfg.is_bus_master_enabled(), Ok(true)) {
        return;
    }
    let _ = cfg.disable_bus_master();
    // Read back: a configuration write is posted, and the line says what the
    // device holds, not what was asked.
    let off = matches!(cfg.is_bus_master_enabled(), Ok(false));
    crate::sys::serial::print(b"[BROKER] claimed device ");
    crate::sys::serial::print_hex(device_id);
    crate::sys::serial::println(if off {
        b" bus master off until its driver turns it on"
    } else {
        b" bus master STILL ON"
    });
}
