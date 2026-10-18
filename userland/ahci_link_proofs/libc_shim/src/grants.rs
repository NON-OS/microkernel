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

//! The register window and interrupt grants. A window's address is never
//! dereferenced on the host: the included register accessors reach the
//! test's model instead.

use crate::calls::Call;
use crate::log::note;
use crate::types::{IrqBindOut, MmioMapOut};

const ENOMEM: i64 = -12;
const MMIO_GRANT: u64 = 0x31;
const IRQ_GRANT: u64 = 0x41;
/// Where the window is said to lie.
const MMIO_VA: u64 = 0x7e00_0000_0000;

pub fn mk_mmio_map(
    _device_id: u64,
    _claim_epoch: u64,
    _bar_index: u32,
    _flags: u32,
    _offset: u64,
    length: u64,
    out: &mut MmioMapOut,
) -> i64 {
    if !note(Call::MmioMap) {
        return ENOMEM;
    }
    *out = MmioMapOut { user_va: MMIO_VA, length, grant_id: MMIO_GRANT };
    0
}

pub fn mk_mmio_unmap(_grant_id: u64) -> i64 {
    note(Call::MmioUnmap);
    0
}

pub fn mk_irq_bind(
    _device_id: u64,
    _claim_epoch: u64,
    _irq_source: u32,
    _flags: u32,
    _vector_count: u32,
    out: &mut IrqBindOut,
) -> i64 {
    if !note(Call::IrqBind) {
        return ENOMEM;
    }
    *out = IrqBindOut { grant_id: IRQ_GRANT, vector: 0x40 };
    0
}

pub fn mk_irq_unbind(_grant_id: u64) -> i64 {
    note(Call::IrqUnbind);
    0
}
