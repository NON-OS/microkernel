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

//! The loader's record, laid out as the loader's `DiskMirror` byte for byte.

pub(super) const MAGIC: [u8; 8] = *b"NONOSDM1";
pub(super) const EXTENTS: usize = 31;

/* Sectors `lba..lba + sectors` (512 bytes each), at physical `phys`. */
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub(super) struct Extent {
    pub(super) lba: u64,
    pub(super) sectors: u64,
    pub(super) phys: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct Record {
    pub(super) magic: [u8; 8],
    pub(super) capacity: u64,
    pub(super) count: u64,
    pub(super) extents: [Extent; EXTENTS],
}

pub(super) const LEN: usize = 24 + 24 * EXTENTS;

const _: () = assert!(core::mem::size_of::<Record>() == LEN);
