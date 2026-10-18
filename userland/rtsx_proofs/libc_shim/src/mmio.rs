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

//! The register BAR's map. The driver's register type is the proofs' own
//! (src/hw/mmio.rs), which hands each access to the modelled reader, so the
//! address here only has to be the one that type is built from.

use crate::events::note;
use crate::types::MmioMapOut;

/// Where the register window is said to be mapped.
pub const MMIO_VA: u64 = 0x7F00_0000;

pub fn mk_mmio_map(
    _device_id: u64,
    _epoch: u64,
    _bar: u32,
    _flags: u32,
    _offset: u64,
    length: u64,
    out: &mut MmioMapOut,
) -> i64 {
    *out = MmioMapOut { user_va: MMIO_VA, length, grant_id: 0x51 };
    note("mmio_map");
    0
}

pub fn mk_mmio_unmap(grant_id: u64) -> i64 {
    note(format!("mmio_unmap {grant_id:#x}"));
    0
}
