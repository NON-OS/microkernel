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

//! The flags and register numbers the driver hands the broker, as the
//! libc defines them. Their files (broker/irq.rs, dma.rs, pci.rs) call the
//! kernel and cannot be mounted here, so the values are written out, and
//! `tests::abi_tests` holds each to the line that defines it there.

pub const MK_IRQ_BIND_MSIX: u32 = 1 << 0;
pub const MK_DMA_MAP_COHERENT: u32 = 1 << 2;
pub const MK_PCI_CFG_COMMAND: u32 = 0x04;
pub const MK_PCI_CMD_MEMORY_SPACE: u16 = 1 << 1;
pub const MK_PCI_CMD_BUS_MASTER: u16 = 1 << 2;
