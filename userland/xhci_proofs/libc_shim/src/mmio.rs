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

//! The register window: whatever the test offers, a `FakeBar` in practice,
//! is what `mk_mmio_map` maps, and giving it back is recorded.

use std::cell::Cell;

use crate::log::{record, Call};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MmioMapOut {
    pub user_va: u64,
    pub length: u64,
    pub grant_id: u64,
}

/// The grant id every mapping gets.
pub const MMIO_GRANT: u64 = 0x10;

thread_local! {
    static WINDOW: Cell<(u64, u64)> = const { Cell::new((0, 0)) };
}

/// The register window `mk_mmio_map` hands out on this thread.
pub fn offer_mmio(user_va: u64, length: u64) {
    WINDOW.with(|w| w.set((user_va, length)));
}

pub fn mk_mmio_map(
    _device_id: u64,
    _claim_epoch: u64,
    _bar_index: u32,
    _flags: u32,
    _offset: u64,
    length: u64,
    out: &mut MmioMapOut,
) -> i64 {
    let (user_va, offered) = WINDOW.with(|w| w.get());
    if user_va == 0 {
        return -19;
    }
    let length = length.min(offered);
    *out = MmioMapOut { user_va, length, grant_id: MMIO_GRANT };
    0
}

pub fn mk_mmio_unmap(grant_id: u64) -> i64 {
    record(Call::MmioUnmap(grant_id));
    0
}
