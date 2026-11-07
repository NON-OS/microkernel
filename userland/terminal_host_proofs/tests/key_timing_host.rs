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

//! `latency` times a key from its press to the frame that shows it, and only
//! a press that asks for a frame (term/terminal/key_timing.rs). On a guest a
//! key that drew nothing was timed to the next frame of any kind, and the
//! Terminal reported a worst key of 11306 ms on an idle window.

#[path = "../../capsule_terminal/src/term/terminal/key_timing.rs"]
mod key_timing;

use key_timing::timed_from;

const APP: &str = include_str!("../../capsule_terminal/src/term/terminal/app_impl.rs");
const ON_EVENT: &str =
    include_str!("../../capsule_terminal/src/term/terminal/app_impl_on_event.rs");
const PAINT: &str = include_str!("../../capsule_terminal/src/term/terminal/app_impl_paint.rs");

#[test]
fn a_press_that_asks_for_a_frame_starts_the_clock() {
    assert_eq!(timed_from(0, 500, true, true), 500);
    assert_eq!(timed_from(120, 500, true, true), 500, "the newest key is timed");
}

#[test]
fn any_other_event_leaves_the_clock_as_it_was() {
    for was in [0, 120] {
        assert_eq!(timed_from(was, 500, true, false), was, "a key that drew nothing");
        assert_eq!(timed_from(was, 500, false, true), was, "a click or a wheel");
        assert_eq!(timed_from(was, 500, false, false), was);
    }
}

/// Over a run of events, a frame is timed only from a key that asked for
/// one, and never from a key that asked for none, however long the gap.
#[test]
fn a_frame_is_timed_only_from_a_key_that_asked_for_it() {
    // (at, key-down, asked for a frame, a frame was painted after it)
    let run = [
        (10, true, false, false),
        (11_000, false, false, true),
        (11_010, true, true, true),
        (11_020, true, false, false),
        (20_000, false, true, true),
    ];
    let (mut at, mut timed) = (0i64, Vec::new());
    for (now, key, repaint, painted) in run {
        at = timed_from(at, now, key, repaint);
        if painted && at > 0 {
            timed.push(now - at);
            at = 0;
        }
    }
    assert_eq!(timed, vec![0], "only the key at 11010, shown in its own frame");
}

#[test]
fn the_event_path_times_through_the_rule_and_the_paint_ends_it() {
    let key = APP.find("let key = event.kind == nonos_app_skeleton::InputKind::KeyDown;");
    let before = APP.find("let now = if key { nonos_libc::mk_uptime_ms().max(1) } else { 0 };");
    assert!(key.is_some() && key < before, "the clock is read for a key-down alone");
    let handled = APP.find("let outcome = self.on_event_inner(event);");
    let rule = APP.find("self.key_at = super::key_timing::timed_from(was, now, key, repaint);");
    assert!(
        before.is_some() && before < handled && handled < rule,
        "the press read before it is handled"
    );
    assert!(APP.contains("let repaint = outcome == EventOutcome::Repaint;"));
    assert!(!ON_EVENT.contains("key_at"), "no other place starts the clock");
    assert!(PAINT.contains("self.cur().key_lat.push(ms as u32);\n            self.key_at = 0;"));
}
