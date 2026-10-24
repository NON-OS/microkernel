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

//! What is handed out when the seeded RNG cannot answer: the hardware
//! entropy source if it gives a value, and otherwise the time counter mixed
//! with a count, which is weak. The first weak value of a boot is said on
//! the serial line, once, on every profile, so a boot shows whether any
//! caller ever reaches it.

use super::super::entropy::get_entropy64_secure;
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

static COUNTER: AtomicU64 = AtomicU64::new(0);
static TOLD: AtomicBool = AtomicBool::new(false);

/// A weak value: the time counter and a count. Said once per boot.
pub(super) fn weak_u64() -> u64 {
    if !TOLD.swap(true, Ordering::Relaxed) {
        crate::sys::serial::print(b"[RNG] fallback used: weak bytes (entropy unavailable)\n");
    }
    crate::arch::read_time_counter() ^ COUNTER.fetch_add(1, Ordering::Relaxed)
}

pub(super) fn fill_with_fallback_secure(buf: &mut [u8]) {
    for chunk in buf.chunks_mut(8) {
        let v = get_entropy64_secure().unwrap_or_else(|_| weak_u64());
        chunk.copy_from_slice(&v.to_le_bytes()[..chunk.len()]);
    }
}
