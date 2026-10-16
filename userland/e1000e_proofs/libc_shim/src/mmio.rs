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

//! The BAR map: the grant `mk_mmio_map` reports, at most the length a test
//! allows, so a window shorter than asked can be handed to the driver.

/// The MMIO grant as `mk_mmio_map` reports it; a rollback rebuilds the
/// register window from it.
pub struct MmioMapOut {
    pub user_va: u64,
    pub length: u64,
    pub grant_id: u64,
}

thread_local! {
    static MAPPED: std::cell::Cell<u64> = const { std::cell::Cell::new(u64::MAX) };
}

/// The most the next BAR maps report, whatever length was asked.
pub fn map_at_most(len: u64) {
    MAPPED.with(|m| m.set(len));
}

pub fn mk_mmio_map(
    _d: u64,
    _e: u64,
    _bar: u32,
    _f: u32,
    _o: u64,
    len: u64,
    out: &mut MmioMapOut,
) -> i64 {
    let length = MAPPED.with(|m| m.get()).min(len);
    *out = MmioMapOut { user_va: 0x4000_0000, length, grant_id: 20 };
    0
}
