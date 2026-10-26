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

//! The kernel's seeded RNG, as the samplers call it: a fixed stream that a
//! test can make fail from its Nth draw on, on this thread only. Only the
//! refusing calls exist here, so a sampler that used another would not build.

use std::cell::Cell;

mod derived;
pub(crate) use derived::{random_range_secure, random_u32_secure};

thread_local! {
    static STATE: Cell<u64> = const { Cell::new(0x9e37_79b9_7f4a_7c15) };
    static DRAWS: Cell<u64> = const { Cell::new(0) };
    static FAIL_FROM: Cell<Option<u64>> = const { Cell::new(None) };
    static ONLY: Cell<bool> = const { Cell::new(false) };
}

/// Draws from the `n`th on (counting from 1) fail; `None` makes all work.
pub fn fail_from(n: Option<u64>) {
    DRAWS.set(0);
    FAIL_FROM.set(n);
    ONLY.set(false);
}

/// Only the `n`th draw fails; the ones after it work again.
pub fn fail_only(n: u64) {
    fail_from(Some(n));
    ONLY.set(true);
}

/// How many secure draws this thread has asked for since `fail_from`.
pub fn draws() -> u64 {
    DRAWS.get()
}

pub(crate) fn next() -> Result<u64, ()> {
    let n = DRAWS.get() + 1;
    DRAWS.set(n);
    if FAIL_FROM.get().is_some_and(|from| n == from || (n > from && !ONLY.get())) {
        return Err(());
    }
    let mut z = STATE.get().wrapping_add(0x9e37_79b9_7f4a_7c15);
    STATE.set(z);
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    Ok(z ^ (z >> 31))
}

pub(crate) fn fill_random_bytes_secure(buf: &mut [u8]) -> Result<(), ()> {
    for chunk in buf.chunks_mut(8) {
        chunk.copy_from_slice(&next()?.to_le_bytes()[..chunk.len()]);
    }
    Ok(())
}
