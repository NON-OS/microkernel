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

//! The SWSTYLE 2 descriptor: sixteen bytes, 32-bit buffer address. Receive
//! and transmit share the shape; `misc` is the received length (RMD2) on one
//! and the error bits (TMD2) on the other.

#[repr(C)]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Desc {
    pub addr: u32,
    pub bcnt: u16,
    pub status: u16,
    pub misc: u32,
    pub reserved: u32,
}

const _: () = assert!(core::mem::size_of::<Desc>() == 16);
