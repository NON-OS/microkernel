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

//! The PCI Command value written after the claim. Memory Space has to be on
//! for BAR0 to answer and Bus Master for the queues to reach the rings;
//! firmware that never booted from the port may leave either off. The bits
//! the record's BARs already decode are kept.

use nonos_libc::{MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_MEMORY_SPACE};

pub fn command_value(record_bits: u16) -> u16 {
    record_bits | MK_PCI_CMD_MEMORY_SPACE | MK_PCI_CMD_BUS_MASTER
}
