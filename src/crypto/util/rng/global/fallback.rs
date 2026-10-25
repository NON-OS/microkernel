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
//! with a count, which is weak. The serial line says, once a boot each, the
//! first draw made before seeding (with the source line that asked) and the
//! first weak value, so a boot shows whether any caller reaches either.

use super::super::entropy::get_entropy64_secure;
use core::panic::Location;
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

static COUNTER: AtomicU64 = AtomicU64::new(0);
static TOLD: AtomicBool = AtomicBool::new(false);
static TOLD_PRE_SEED: AtomicBool = AtomicBool::new(false);

/// Says, once a boot, which source line drew before the RNG was seeded, as
/// one write so no other output splits it. A path too long is cut short.
pub(super) fn say_pre_seed(at: &Location<'_>) {
    if TOLD_PRE_SEED.swap(true, Ordering::Relaxed) {
        return;
    }
    let (mut line, mut len) = ([0u8; 160], 0);
    let mut put = |bytes: &[u8]| {
        let n = bytes.len().min(line.len() - 12 - len);
        line[len..len + n].copy_from_slice(&bytes[..n]);
        len += n;
    };
    put(b"[RNG] pre-seed draw by ");
    put(at.file().as_bytes());
    // Source lines count from 1, so a digit is always written.
    let mut digits = [b':'; 11];
    let (mut n, mut i) = (at.line(), digits.len());
    while n != 0 {
        i -= 1;
        digits[i] = b'0' + (n % 10) as u8;
        n /= 10;
    }
    let tail = &digits[i - 1..];
    line[len..len + tail.len()].copy_from_slice(tail);
    len += tail.len();
    line[len] = b'\n';
    crate::sys::serial::print(&line[..=len]);
}

/// A weak value: the time counter and a count. Said once per boot.
pub(super) fn weak_u64() -> u64 {
    if !TOLD.swap(true, Ordering::Relaxed) {
        crate::sys::serial::print(b"[RNG] fallback used: weak bytes (entropy unavailable)\n");
    }
    crate::arch::read_time_counter() ^ COUNTER.fetch_add(1, Ordering::Relaxed)
}

#[track_caller]
pub(super) fn fill_with_fallback_secure(buf: &mut [u8]) {
    say_pre_seed(Location::caller());
    for chunk in buf.chunks_mut(8) {
        let v = get_entropy64_secure().unwrap_or_else(|_| weak_u64());
        chunk.copy_from_slice(&v.to_le_bytes()[..chunk.len()]);
    }
}
