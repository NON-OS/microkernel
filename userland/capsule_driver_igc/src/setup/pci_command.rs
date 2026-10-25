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

//! The PCI Command value. Memory Space has to be on for BAR0 to answer, and
//! is always written. Bus Master is off from the claim until the part is
//! reset and both rings name this driver's frames, and off again before the
//! device goes back: a part left mastering by firmware or an earlier attempt
//! must not reach memory before it is stopped, nor after its frames are
//! freed. Only allowlisted bits go in, so the broker never refuses the value
//! whatever firmware left. `pci.rs` writes it by the masked form, naming
//! both bits, so "off" turns Bus Master off and every other bit stays as it
//! is; the plain form ORs the value into the register and could only turn
//! bits on (src/hardware/broker/pci/change.rs).

use nonos_libc::{MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_MEMORY_SPACE};

/// The Command bits the value sets and clears.
pub const COMMAND_MASK: u16 = MK_PCI_CMD_MEMORY_SPACE | MK_PCI_CMD_BUS_MASTER;

pub fn command_value(bus_master: bool) -> u16 {
    MK_PCI_CMD_MEMORY_SPACE | if bus_master { MK_PCI_CMD_BUS_MASTER } else { 0 }
}
