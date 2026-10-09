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

//! Date, performance.now() and the Math.random seed read the clocks the
//! browser lends the engine. On the bare target the C library's time calls
//! had nothing behind them: every page was told it was 1 January 1970,
//! performance.now() never moved, and Math.random gave every page the same
//! numbers. The hosted build reads the lent clocks through the same path
//! (NJS_HOST_CLOCKS), so this proves what the capsule runs.
//!
//! Run with `cargo test --release --features hosted`.

use std::sync::atomic::{AtomicI64, AtomicU64, Ordering};

use nonos_qjs::{Engine, Limits};

static MONO: AtomicU64 = AtomicU64::new(0);
static WALL: AtomicI64 = AtomicI64::new(0);

extern "C" fn clock() -> u64 {
    MONO.load(Ordering::SeqCst)
}

extern "C" fn wall() -> i64 {
    WALL.load(Ordering::SeqCst)
}

fn engine() -> Engine {
    let limits = Limits { memory: 8 * 1024 * 1024, stack: 512 * 1024, budget_ms: 0, clock, wall };
    Engine::with_limits(limits).expect("an engine")
}

/* One test: the lent clocks are process wide, as they are in the capsule. */
#[test]
fn a_page_reads_the_clocks_it_is_lent() {
    // 2026-10-03T12:34:56.789Z
    WALL.store(1_791_030_896_789, Ordering::SeqCst);
    MONO.store(5_000, Ordering::SeqCst);
    let e = engine();

    assert_eq!(e.eval("Date.now()"), "1791030896789");
    assert_eq!(e.eval("new Date().toISOString()"), "2026-10-03T12:34:56.789Z");

    // performance.now() counts from the engine's making, on the monotonic
    // clock, and moves when it does.
    assert_eq!(e.eval("performance.now()"), "0");
    MONO.store(5_250, Ordering::SeqCst);
    assert_eq!(e.eval("performance.now()"), "250");
    assert_eq!(e.eval("performance.timeOrigin"), "5000");

    // The wall clock moves Date on its own.
    WALL.store(1_791_030_956_789, Ordering::SeqCst);
    assert_eq!(e.eval("new Date().toISOString()"), "2026-10-03T12:35:56.789Z");

    // Math.random is seeded from the wall clock at the engine's making, so
    // a page made at another moment draws other numbers.
    let first = e.eval("Math.random()");
    let other = engine();
    let second = other.eval("Math.random()");
    assert_ne!(first, second, "two pages made at different moments draw differently");
}
