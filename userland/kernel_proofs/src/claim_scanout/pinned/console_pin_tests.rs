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

//! The console's count and hold read whole (L79). The checks beside these
//! read `drawable` in order, which let a line slip in between, and did not
//! read the wait, HELD's store, `taken` or the hold's clearing at all; the
//! model's stop, drawer, bootfb wait and end each stand for one of them.

use super::super::glue_text::{body, count, source};

const HELD: &str = "kernel_core/init/framebuffer/held.rs";
const DRAWING: &str = "kernel_core/init/framebuffer/drawing.rs";
const HOLD: &str = "hardware/broker/bootfb/hold.rs";

#[test]
fn a_draw_is_counted_until_it_ends_and_the_wait_is_for_none_left() {
    let want = "{ if stopped() { return None; } DRAWS.fetch_add(1, SeqCst); \
        let fb = framebuffer_state().filter(|_| !stopped()); \
        if fb.is_none() { DRAWS.fetch_sub(1, SeqCst); } fb.map(Drawing) }";
    assert_eq!(body(DRAWING, "drawable"), want);
    let wait = "{ while DRAWS.load(SeqCst) != 0 { crate::smp::serve_shootdowns(); \
        core::hint::spin_loop(); } }";
    assert_eq!(body(DRAWING, "wait_quiet"), wait);
    // Counted in once and out on its two ways: backed out, or dropped.
    let text = source(DRAWING);
    assert_eq!(count(&text, "DRAWS.fetch_add("), 1, "another count in");
    assert_eq!(count(&text, "DRAWS.fetch_sub("), 2, "another count out");
    assert_eq!(count(&text, "DRAWS.store("), 0, "the count overwritten");
}

#[test]
fn the_hold_is_stored_seqcst_and_cleared_only_as_bootfbs_entry_goes() {
    assert_eq!(body(HELD, "set_held"), "{ HELD.store(held, SeqCst); }");
    let text = source(HELD);
    for (what, n) in [("HELD.store(", 1), ("GONE.store(", 1), ("TAKERS.fetch_", 2)] {
        assert_eq!(count(&text, what), n, "{what}");
    }
    assert_eq!(count(&text, "TAKERS.store("), 0, "the takeovers' count overwritten");
    let released = "{ let fb = is_boot_framebuffer(device_id); \
        if fb { set_held(false); } fb }";
    assert_eq!(body(HOLD, "released"), released);
}

#[test]
fn a_release_during_a_takeover_says_the_console_does_not_come_back() {
    let taken = "{ TAKERS.load(SeqCst) > 0 || GONE.load(SeqCst) }";
    assert_eq!(body(HELD, "taken"), taken);
    let say = body(HOLD, "say");
    let order = ["if held {", "} else if taken() {", "let go: a display driver", "} else {"];
    super::super::glue_text::in_order(&say, &order);
}
