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

//! The kernel's seeded RNG, as its callers reach it, on this thread only. A
//! test sets which draws refuse (as before seeding), counts the
//! draws, and reads back each answer given. The weak paths the callers must
//! not use are here too; each counts its calls (`weak`).

use std::cell::{Cell, RefCell};
use std::sync::atomic::{AtomicU64, Ordering};

#[path = "../../../../src/crypto/util/rng/error.rs"]
mod error;
pub use error::{RngError, RngResult};

static STREAMS: AtomicU64 = AtomicU64::new(1);

thread_local! {
    static STATE: Cell<u64> = Cell::new(STREAMS.fetch_add(1, Ordering::Relaxed) << 40);
    static DRAWS: Cell<u64> = const { Cell::new(0) };
    static FAIL_FROM: Cell<Option<u64>> = const { Cell::new(None) };
    static ONLY: Cell<bool> = const { Cell::new(false) };
    static WEAK: Cell<u64> = const { Cell::new(0) };
    static GIVEN: RefCell<Vec<Vec<u8>>> = const { RefCell::new(Vec::new()) };
}

/// Every draw refuses, as before the RNG is seeded.
pub fn refuse() {
    fail_from(Some(1));
}

/// How many seeded draws were asked for since `fail_from`.
pub fn draws() -> u64 {
    DRAWS.get()
}

/// How many times a weak path was called since `fail_from`.
pub fn weak() -> u64 {
    WEAK.get()
}

/// The bytes each answered draw gave, in order.
pub fn given() -> Vec<Vec<u8>> {
    GIVEN.with_borrow(Clone::clone)
}

fn next_word() -> u64 {
    let mut z = STATE.get().wrapping_add(0x9e37_79b9_7f4a_7c15);
    STATE.set(z);
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

include!("rng_calls.rs");
include!("rng_weak.rs");
