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

//! Low pool pressure in log DMA. A Wi-Fi card that cannot map its rings on a
//! real machine shows up as a join that never starts; this line says the pool
//! was running out before that, and who was left with how much.

use core::sync::atomic::{AtomicUsize, Ordering};

use super::bitmap::count_used;
use super::low32::LOW32_POOL;
use super::sizing::{low32_floor, pressure_quarter};

/// The highest quarter already said, so a busy pool says each step once.
static SAID: AtomicUsize = AtomicUsize::new(0);

/// Say the pool's use once it reaches three quarters, and again at each
/// higher quarter; it may say so again after falling back under half.
pub(super) fn note(used: usize, total: usize) {
    let quarter = pressure_quarter(used, total);
    if quarter <= 1 {
        SAID.store(0, Ordering::Relaxed);
        return;
    }
    if quarter < 3 || SAID.fetch_max(quarter, Ordering::Relaxed) >= quarter {
        return;
    }
    crate::sys::serial::print(b"[DMA] low32 pool pressure: used=");
    crate::sys::serial::print_dec(used as u64);
    crate::sys::serial::print(b" of ");
    crate::sys::serial::print_dec(total as u64);
    crate::sys::serial::println(b" pages");
}

/// Say how the low pool stands, for a map that found no memory.
pub(in crate::hardware::broker::dma) fn say_low32() {
    let (used, total) = {
        let pool = LOW32_POOL.lock();
        (count_used(&pool.used), pool.pages)
    };
    crate::sys::serial::print(b"[DMA] low32 pool used=");
    crate::sys::serial::print_dec(used as u64);
    crate::sys::serial::print(b" of ");
    crate::sys::serial::print_dec(total as u64);
    crate::sys::serial::print(b" floor=");
    crate::sys::serial::print_dec(low32_floor(total) as u64);
    crate::sys::serial::println(b"");
}
