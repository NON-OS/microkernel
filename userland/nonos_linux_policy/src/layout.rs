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

//! Where each field sits in the 256 bytes. Everything is little-endian.

pub const LEN: usize = 256;
pub const MAGIC: [u8; 8] = *b"NLXPOL01";
pub const VERSION: u16 = 1;

pub(crate) const AT_MAGIC: usize = 0;
pub(crate) const AT_VERSION: usize = 8;
/// Six zero bytes between the version and the program.
pub(crate) const AT_PAD_HEAD: usize = 10;
pub(crate) const AT_PROGRAM: usize = 16;
pub(crate) const AT_MASK: usize = 48;
pub(crate) const AT_LISTING: usize = 56;
pub(crate) const AT_RELEASE: usize = 88;
pub(crate) const AT_NET_DESTS: usize = 120;
pub(crate) const AT_SYSCALLS: usize = 152;
/// From the end of the syscall set to the end, all zero.
pub(crate) const AT_PAD_TAIL: usize = 217;
