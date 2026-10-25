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

//! The 21x4x descriptor: sixteen bytes, status (with OWN) then control
//! (sizes and flags) then two 32-bit buffer addresses. Receive and transmit
//! share the shape; this driver uses buffer 1 only.

#[repr(C)]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Desc {
    pub status: u32,
    pub control: u32,
    pub buf1: u32,
    pub buf2: u32,
}

const _: () = assert!(core::mem::size_of::<Desc>() == 16);
