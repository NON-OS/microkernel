// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! Cleaning by address, driven by the arithmetic in `geometry`.

use super::geometry::{dminline_bytes, lines};

/// Every line of `[start, start + len)` written back to the point of
/// coherency and dropped (DC CIVAC), then a DSB SY so it is done before the
/// caller goes on. False, having done nothing, when the range wraps.
pub fn clean_invalidate_to_poc(start: u64, len: u64) -> bool {
    let ctr: u64;
    // SAFETY: CTR_EL0 is readable at the EL UEFI runs at.
    unsafe { core::arch::asm!("mrs {}, ctr_el0", out(reg) ctr, options(nomem, nostack)) };
    let line = dminline_bytes(ctr);
    let Some((first, count)) = lines(start, len, line) else {
        return false;
    };
    for i in 0..count {
        let va = first + i * line;
        // SAFETY: a maintenance operation on a mapped address; it changes no
        // memory contents, only where they are held.
        unsafe { core::arch::asm!("dc civac, {}", in(reg) va, options(nostack, preserves_flags)) };
    }
    // SAFETY: a barrier only.
    unsafe { core::arch::asm!("dsb sy", options(nostack, preserves_flags)) };
    true
}
