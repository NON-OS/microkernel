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

//! The console the model's drawer and stop stand for, read (L79): the stop
//! is HELD, GONE or any takeover counted; a takeover counts itself then
//! waits out the draws; a draw counts itself and reads the stop again;
//! bootfb's hold is set under the lock, the draws waited out after it.

use super::glue_text::{body, in_order};

const HELD: &str = "kernel_core/init/framebuffer/held.rs";
const DRAWING: &str = "kernel_core/init/framebuffer/drawing.rs";
const HOLD: &str = "hardware/broker/bootfb/hold.rs";

#[test]
fn the_stop_counts_every_takeover_and_latches_gone_for_good() {
    let stopped = "{ HELD.load(SeqCst) || TAKERS.load(SeqCst) > 0 || GONE.load(SeqCst) }";
    assert_eq!(body(HELD, "stopped"), stopped);
    let stop = "{ TAKERS.fetch_add(1, SeqCst); super::drawing::wait_quiet(); }";
    assert_eq!(body(HELD, "stop_drawing"), stop);
    assert_eq!(body(HELD, "end_takeover"), "{ TAKERS.fetch_sub(1, SeqCst); }");
    assert_eq!(body(HELD, "set_gone"), "{ GONE.store(true, SeqCst); }");
    assert_eq!(body(HELD, "surrendered"), "{ GONE.load(SeqCst) }");
}

#[test]
fn a_draw_counts_itself_before_it_reads_the_stop_again() {
    in_order(
        &body(DRAWING, "drawable"),
        &[
            "{ if stopped() { return None; }",
            "DRAWS.fetch_add(1, SeqCst);",
            "let fb = framebuffer_state().filter(|_| !stopped());",
            "if fb.is_none() { DRAWS.fetch_sub(1, SeqCst); } fb.map(Drawing) }",
        ],
    );
    let drop = "{ core::sync::atomic::fence(SeqCst); DRAWS.fetch_sub(1, SeqCst); }";
    assert_eq!(body(DRAWING, "drop"), drop);
}

#[test]
fn bootfbs_hold_is_set_under_the_lock_and_the_draws_waited_out_after() {
    let want = "{ if is_boot_framebuffer(id) { set_held(true); drop(claims); wait_quiet(); \
        say(true); } }";
    assert_eq!(body(HOLD, "admitted"), want);
}
