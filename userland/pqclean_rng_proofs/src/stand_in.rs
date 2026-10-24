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

//! What stands in for the kernel around the mounted files. The seeded RNG
//! answers with a counter, or fails on demand: from the Nth draw on, or on
//! only the Nth. `fill_random_bytes` is the old weak path, which always
//! answers. The hardware stand-ins are in `stand_in_hw`.

use std::cell::RefCell;
use std::sync::Mutex;

thread_local! {
    static DRAWS: RefCell<(u64, Option<(u64, bool)>)> = const { RefCell::new((0, None)) };
}

/// A draw failing on any thread refuses every call running meanwhile, by
/// design, so the tests that make draws fail take turns through this.
static TURN: Mutex<()> = Mutex::new(());

pub fn turn() -> std::sync::MutexGuard<'static, ()> {
    TURN.lock().unwrap_or_else(|p| p.into_inner())
}

/// Fail draw `n` (counting from 1) of this thread, and every later one when
/// `onward`; `None` makes every draw work.
pub fn fail_draws(at: Option<(u64, bool)>) {
    DRAWS.with(|d| *d.borrow_mut() = (0, at));
}

#[allow(clippy::result_unit_err)]
pub fn fill_random_bytes_secure(buf: &mut [u8]) -> Result<(), ()> {
    let fail = DRAWS.with(|d| {
        let mut d = d.borrow_mut();
        d.0 += 1;
        matches!(d.1, Some((n, onward)) if d.0 == n || (onward && d.0 > n))
    });
    if fail {
        return Err(());
    }
    fill_random_bytes(buf);
    Ok(())
}

/// The old path: always answers, which is exactly what must not reach a key.
pub fn fill_random_bytes(buf: &mut [u8]) {
    buf.iter_mut().enumerate().for_each(|(i, b)| *b = (i as u8).wrapping_mul(97) ^ 0x5a);
}

pub use crate::stand_in_hw::{entropy_works, get_entropy64_secure, SERIAL};
