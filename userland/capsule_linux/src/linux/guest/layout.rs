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

//! Where things sit in a guest's address space.

/// The heap, growing up from here as `brk` moves.
pub const BRK_BASE: u64 = 0x0000_1000_0000;

/// Anonymous and file mappings, growing up from here: high above the images
/// and the stack, with room to the top of user space for large reservations.
pub const MMAP_BASE: u64 = 0x0000_0001_0000_0000;

/// Where the loader biases a position-independent executable.
pub const EXEC_BASE: u64 = 0x0000_4000_0000;

/// Where its interpreter goes, far enough above the executable that
/// neither can grow into the other.
pub const INTERP_BASE: u64 = 0x0000_5000_0000;

/// The stack top a guest wakes on, above everything it maps for itself.
pub const STACK_TOP: u64 = 0x0000_7FFF_F000;

/// The stack a guest gets.
pub const STACK_SIZE: u64 = 1 << 20;

/// The break may not reach the images above it.
pub const BRK_LIMIT: u64 = 0x0000_2000_0000;

/// A mapping may not reach the top of the window left below the stack.
pub const MMAP_LIMIT: u64 = 0x0000_7F00_0000_0000;

/// The ceiling for any mapping: one page below the top of user space. A
/// runtime that reserves a large address range, as Go's page allocator does,
/// needs the room, and a reservation backs no frames until it is touched.
pub const USER_MAX: u64 = 0x0000_7FFF_FFFF_F000;
