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

//! The descriptor shapes, sixteen bytes each. A transmit or receive
//! descriptor is a 64-bit buffer address and two control words; a
//! completion is four words, with its generation bit at the top of the last.

#[repr(C)]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Desc {
    pub addr: u64,
    pub d2: u32,
    pub d3: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Comp {
    pub d0: u32,
    pub d1: u32,
    pub d2: u32,
    pub d3: u32,
}

const _: () = assert!(core::mem::size_of::<Desc>() == 16);
const _: () = assert!(core::mem::size_of::<Comp>() == 16);

/// Whether a completion is the driver's: its gen bit equals the ring's.
///
/// # Safety
/// `c` points at a completion inside a live ring grant.
pub unsafe fn completed(c: *const Comp, gen: u32) -> bool {
    use crate::constants::ring::COMP_GEN_SHIFT;
    let d3 = core::ptr::read_volatile(core::ptr::addr_of!((*c).d3));
    d3 >> COMP_GEN_SHIFT == gen
}
