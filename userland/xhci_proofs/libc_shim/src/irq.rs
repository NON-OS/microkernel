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

//! Interrupt binding. The device offers the kinds a test says it has (none
//! by default), and every kind asked for is remembered. What becomes of a
//! bound grant is `irq_wait`.

use std::cell::Cell;

use crate::irq_wait::keep_bound;

pub const MK_IRQ_BIND_MSIX: u32 = 1 << 0;
pub const MK_IRQ_BIND_MSI: u32 = 1 << 1;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IrqBindOut {
    pub grant_id: u64,
    pub vector: u64,
}

thread_local! {
    static KINDS: Cell<u32> = const { Cell::new(0) };
    static TRIED: Cell<u32> = const { Cell::new(0) };
    static BOUND: Cell<u64> = const { Cell::new(0) };
}

/// The `MK_IRQ_BIND_*` kinds the device binds.
pub fn offer_irq(kinds: u32) {
    KINDS.with(|k| k.set(kinds));
}

/// Every kind the driver asked to bind, ORed together.
pub fn irq_tried() -> u32 {
    TRIED.with(|t| t.get())
}

pub fn mk_irq_bind(_d: u64, _e: u64, _src: u32, flags: u32, _n: u32, out: &mut IrqBindOut) -> i64 {
    TRIED.with(|t| t.set(t.get() | flags));
    if KINDS.with(|k| k.get()) & flags == 0 {
        return -19;
    }
    // Each bind its own grant, its kind in the low bits.
    let n = BOUND.with(|b| b.replace(b.get() + 1));
    let grant_id = 0x40 + (n << 8) + flags as u64;
    keep_bound(grant_id);
    *out = IrqBindOut { grant_id, vector: 0x30 };
    0
}
