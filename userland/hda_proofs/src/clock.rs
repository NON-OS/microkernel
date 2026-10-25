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
//! The host's stand-in for the capsule's `clock` module: the same two
//! functions, on `std::time`, how many times each thread has slept, and a
//! clock a test can set by hand on its own thread, or step a model on.

use std::cell::Cell;
use std::sync::OnceLock;
use std::time::{Duration, Instant};

mod poll;
pub use poll::{on_poll, Polled};

thread_local! {
    static SLEPT: Cell<u64> = const { Cell::new(0) };
    static SET: Cell<Option<u64>> = const { Cell::new(None) };
}

fn epoch() -> Instant {
    static EPOCH: OnceLock<Instant> = OnceLock::new();
    *EPOCH.get_or_init(Instant::now)
}

pub fn now_ms() -> u64 {
    poll::step();
    if let Some(ms) = SET.with(Cell::get) {
        return ms;
    }
    // The driver polls this between reads of a register a device model updates
    // from its own thread. `nix flake check` builds every proof crate at once,
    // so on a loaded builder a busy-spinning poll can hold its core for the
    // whole deadline while the model never gets one, and a part that does answer
    // reads as a timeout. Yielding here hands the core over on each poll, so the
    // model is scheduled and answers; real time still bounds a wait that no
    // model ever ends, so the timeout proofs keep biting.
    std::thread::yield_now();
    epoch().elapsed().as_millis() as u64
}

pub fn pause_ms(ms: u64) {
    SLEPT.with(|s| s.set(s.get() + 1));
    SET.with(|t| t.set(t.get().map(|now| now + ms)));
    std::thread::sleep(Duration::from_millis(ms));
}

/// How many times this thread has slept in `pause_ms`: each one a wait
/// handing its core back rather than spinning.
pub fn sleeps() -> u64 {
    SLEPT.with(Cell::get)
}

/// Read `ms` on this thread's clock from now on, and move on only when set
/// again or slept on, so a test says when a millisecond tick lands.
pub fn set_ms(ms: u64) {
    SET.with(|t| t.set(Some(ms)));
}
