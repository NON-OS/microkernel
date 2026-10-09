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

//! The real QuickJS, built for the host, stops runaway code and survives it.
//!
//! Run with `cargo test --release --features hosted`.

use std::sync::OnceLock;
use std::time::Instant;

use nonos_qjs::{Engine, Limits, Stop};

static START: OnceLock<Instant> = OnceLock::new();

extern "C" fn clock() -> u64 {
    START.get_or_init(Instant::now).elapsed().as_millis() as u64
}

extern "C" fn wall() -> i64 {
    1_767_225_600_000 + clock() as i64
}

const BUDGET_MS: u64 = 300;

fn engine(memory: usize) -> Engine {
    let limits = Limits { memory, stack: 1024 * 1024, budget_ms: BUDGET_MS, clock, wall };
    Engine::with_limits(limits).expect("an engine")
}

/* The engine keeps the stop flag process wide, as the capsule runs one page
 * at a time; these run one after another in a single test so no other
 * test's stop is read as this one's. */
#[test]
fn runaway_code_is_stopped_and_the_engine_stays_usable() {
    let e = engine(32 * 1024 * 1024);
    assert_eq!(e.eval("1+1"), "2");
    assert_eq!(e.take_stop(), None, "nothing stopped yet");

    let at = Instant::now();
    let out = e.eval("var n=0; while(true){n++}");
    let took = at.elapsed().as_millis() as u64;
    assert!(out.contains("interrupted"), "the loop was cut off: {out}");
    assert!((BUDGET_MS..BUDGET_MS * 4).contains(&took), "stopped near its budget: {took} ms");
    assert_eq!(e.take_stop(), Some(Stop::Time));
    assert_eq!(e.take_stop(), None, "reported once");

    let caught = e.eval("var r='no'; try { while(true){} } catch (x) { r='caught' } r");
    assert!(caught.contains("interrupted"), "a script cannot catch the stop: {caught}");
    assert_eq!(e.take_stop(), Some(Stop::Time));

    assert_eq!(e.eval("n > 0 ? 'ran' : 'did not'"), "ran", "the page's state survives");
    assert_eq!(e.eval("[1,2,3].map(function(x){return x*2}).join()"), "2,4,6");

    let deep = e.eval("function f(n){return f(n+1)+1} try { f(0) } catch (x) { String(x) }");
    assert!(deep.contains("stack"), "deep recursion stops at the stack limit: {deep}");
    assert_eq!(e.eval("'still here'"), "still here");

    let big = e.eval("var a=[]; while(true){a.push(new Array(100000).fill(1))}");
    assert!(big.contains("out of memory"), "the heap limit holds: {big}");
    assert_eq!(e.take_stop(), Some(Stop::Memory));
    assert_eq!(e.eval("a = null; 'freed'"), "freed");

    /* A page's first run may be given longer, and the budget goes back. */
    e.set_budget(BUDGET_MS * 3);
    let at = Instant::now();
    let out = e.eval("var t=performance.now(); while(performance.now()-t < 600){} 'done'");
    assert_eq!(out, "done", "twice the old budget fits in the longer one");
    let took = at.elapsed().as_millis() as u64;
    assert!(took >= 590, "ran {took} ms, twice the old budget");
    assert_eq!(e.take_stop(), None);
    e.set_budget(BUDGET_MS);
    let out = e.eval("while(true){}");
    assert!(out.contains("interrupted"), "the short budget is back: {out}");
    assert_eq!(e.take_stop(), Some(Stop::Time));
}
