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

//! The FIFO's register words at its start, then the command ring. Word
//! numbers from svga_reg.h.

pub const FIFO_MIN: usize = 0;
pub const FIFO_MAX: usize = 1;
pub const FIFO_NEXT_CMD: usize = 2;
pub const FIFO_STOP: usize = 3;
/// SVGA_FIFO_BUSY, present only with SVGA_CAP_EXTENDED_FIFO.
pub const FIFO_BUSY: usize = 290;
/// The four registers every SVGA II FIFO has.
pub const BASIC_REGS: u32 = 4;
/// The ring starts at least a page in, as Linux's vmwgfx puts it.
pub const MIN_START: u32 = 4096;
/// QEMU refuses a ring shorter than 10 KiB.
pub const MIN_RING: u32 = 10 * 1024;
pub const CMD_UPDATE: u32 = 1;
/// How long a full ring is waited on before a frame is dropped.
pub const SPACE_WAIT_MS: u64 = 100;
