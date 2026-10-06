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

//! The PCI Command value the claim writes.

use nonos_libc::{MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_MEMORY_SPACE};

/*
 * Only Memory Space and Bus Master go in. The broker takes a value carrying a
 * bit outside its allowlist verbatim and refuses it if any such bit differs
 * from what the firmware left (src/hardware/broker/pci/command.rs). The
 * record marks I/O Space for the card's port BAR, and UEFI firmware commonly
 * leaves I/O decode off, or SERR and parity on: the old word then differed in
 * a bit the driver may not flip, the write was refused and the card never
 * started. The driver reaches the part through its MMIO BAR alone, so I/O
 * decode is never needed, and the bits the firmware set stay as they were.
 */
pub fn command_word(record_bits: u16) -> u16 {
    (record_bits & MK_PCI_CMD_MEMORY_SPACE) | MK_PCI_CMD_BUS_MASTER
}
