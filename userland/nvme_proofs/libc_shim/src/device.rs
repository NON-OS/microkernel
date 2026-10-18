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

//! The claim, BAR0 map and MSI-X bind bring-up makes, each recorded and
//! refused when the test asks (`refuse`). BAR0 maps the register block the
//! test's controller lives in (`set_bar0`); the claim's epoch is 1.

use std::cell::Cell;

use crate::calls::answer;

#[repr(C)]
pub struct MmioMapOut {
    pub user_va: u64,
    pub length: u64,
    pub grant_id: u64,
}

#[repr(C)]
pub struct IrqBindOut {
    pub grant_id: u64,
    pub vector: u64,
}

pub const MK_IRQ_BIND_MSIX: u32 = 1 << 0;

thread_local! {
    static BAR0: Cell<u64> = const { Cell::new(0) };
}

/// The register block a BAR0 map hands back.
pub fn set_bar0(va: u64) {
    BAR0.with(|b| b.set(va));
}

pub fn mk_device_claim(device_id: u64) -> i64 {
    match answer("claim", device_id) {
        0 => 1,
        refused => refused,
    }
}

pub fn mk_mmio_map(
    _: u64,
    _: u64,
    _: u32,
    _: u32,
    _: u64,
    length: u64,
    out: &mut MmioMapOut,
) -> i64 {
    let user_va = BAR0.with(Cell::get);
    *out = MmioMapOut { user_va, length, grant_id: 0x10 };
    answer("mmio_map", length)
}

pub fn mk_irq_bind(_: u64, _: u64, _: u32, _: u32, _: u32, out: &mut IrqBindOut) -> i64 {
    *out = IrqBindOut { grant_id: 0x20, vector: 0 };
    answer("irq_bind", 0)
}
