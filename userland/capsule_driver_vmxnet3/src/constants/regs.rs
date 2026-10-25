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

//! The two register pages, every register 32 bits wide on an 8-byte stride.

// BAR1, the virtual device page.
pub const VRRS: usize = 0x00;
pub const UVRS: usize = 0x08;
pub const DSAL: usize = 0x10;
pub const DSAH: usize = 0x18;
pub const CMD: usize = 0x20;
pub const MACL: usize = 0x28;
pub const MACH: usize = 0x30;

// BAR0, the pass-through page.
pub const IMR: usize = 0x000;
pub const TXPROD: usize = 0x600;
pub const RXPROD: usize = 0x800;
pub const RXPROD2: usize = 0xA00;

/// Revision 1 and UPT 1: the layouts every vmxnet3 host offers and QEMU's
/// only ones. Later revisions add features this driver does not use.
pub const REV_1: u32 = 1;
pub const UPT_1: u32 = 1;
