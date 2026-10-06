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

//! Entropy for the proofs: a fixed xorshift stream, so every run draws the
//! same keys and transaction ids. Nothing here is meant to be unpredictable.

use std::sync::atomic::{AtomicU64, Ordering};

static STATE: AtomicU64 = AtomicU64::new(0x9E37_79B9_7F4A_7C15);

pub fn crypto_random(buf: *mut u8, len: usize) -> i64 {
    if buf.is_null() {
        return -22;
    }
    for i in 0..len {
        let mut x = STATE.load(Ordering::SeqCst);
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        STATE.store(x, Ordering::SeqCst);
        // SAFETY: the capsule passes its own buffer of `len` bytes.
        unsafe { *buf.add(i) = x as u8 };
    }
    len as i64
}
