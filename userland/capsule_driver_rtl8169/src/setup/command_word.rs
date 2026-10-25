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

//! The PCI Command word.

use nonos_libc::{MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_MEMORY_SPACE};

/*
 * Memory Space, which the MMIO BAR the driver reaches the part through needs,
 * and Bus Master only when asked. Nothing outside the broker's allowlist goes
 * in: the broker takes a value carrying such a bit verbatim and refuses it
 * if the bit differs from what the firmware left (I/O decode off, SERR or
 * parity on), and then the card never started. Bus Master is off from the
 * claim until the part is reset and both rings name this driver's frames,
 * and off again before the device goes back. `pci.rs` writes the word by
 * the masked form, naming both bits, so "off" turns Bus Master off; the
 * plain form ORs the word into the register and could only turn bits on
 * (src/hardware/broker/pci/change.rs).
 */
pub fn command_word(bus_master: bool) -> u16 {
    MK_PCI_CMD_MEMORY_SPACE | if bus_master { MK_PCI_CMD_BUS_MASTER } else { 0 }
}
