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

use crate::drivers::pci::config::ConfigSpace;

/*
 * Stop a released device from mastering the bus. A driver turns Bus Master
 * Enable on through the config-write allowlist, and nothing turned it off:
 * a device whose driver exited kept its DMA running, reaching all of
 * memory. This write is what stops it.
 */
pub(super) fn stop_bus_master(device_id: u64) {
    let Some(handle) = crate::hardware::broker::pci_index::lookup(device_id) else {
        return;
    };
    let cfg = ConfigSpace::new(handle.address);
    // Read back, so the log says what the device holds, not what was asked.
    let off = cfg.disable_bus_master().is_ok() && matches!(cfg.is_bus_master_enabled(), Ok(false));
    crate::sys::serial::print(b"[BROKER] released device ");
    crate::sys::serial::print_hex(device_id);
    crate::sys::serial::println(if off { b" bus master off" } else { b" bus master STILL ON" });
}
