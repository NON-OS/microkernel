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

/// One `Elf64_Rela`.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Rela {
    pub(crate) offset: u64,
    pub(crate) info: u64,
    pub(crate) addend: u64,
}

/// The only relocation type a static PIE of this kernel carries.
pub(crate) const R_AARCH64_RELATIVE: u64 = 1027;

/// The Image header promises a 2 MiB aligned base (Linux arm64 boot protocol,
/// whose header this image carries); the layout keeps writable data on its
/// own 2 MiB block, which only a 2 MiB aligned bias preserves.
pub(crate) const LOAD_ALIGN: u64 = 2 << 20;

/// Why an image was not relocated. The value is what the entry stub sees.
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Refused {
    LoadMisaligned = 1,
    NotRelative = 2,
    OutsideImage = 3,
    MisalignedPlace = 4,
}

/// Where the image was linked and where it is: `len` bytes from each.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Placement {
    pub(crate) link_base: u64,
    pub(crate) load_base: u64,
    pub(crate) len: u64,
}
