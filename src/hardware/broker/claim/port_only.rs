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

//! A port-only claim's Command register (H-02, confine/port_only.rs).

use crate::drivers::pci::config::ConfigSpace;
use crate::hardware::broker::pci::command_rule::{at_claim, BUS_MASTER, MEMORY_SPACE};

/// A port-only claim's Command register (H-02): Memory Space and Bus Master
/// off (pci::command_rule::at_claim), read back as stop_bus_master_quietly
/// does, and said if either is still on.
pub(in crate::hardware::broker) fn port_only_quietly(device_id: u64) {
    let Some(handle) = crate::hardware::broker::pci_index::lookup(device_id) else {
        return;
    };
    let cfg = ConfigSpace::new(handle.address);
    if let Ok(cmd) = cfg.command() {
        let _ = cfg.set_command(at_claim(cmd, true));
    }
    if !matches!(cfg.command(), Ok(c) if c & (BUS_MASTER | MEMORY_SPACE) == 0) {
        crate::sys::serial::print(b"[BROKER] device ");
        crate::sys::serial::print_hex(device_id);
        crate::sys::serial::println(b" port-only claim: memory space or bus master STILL ON");
    }
}
