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

//! A mirrored output's poll: a moved serial or a stale FLUSH asks SURFACE
//! again, LOST repaints, silence waits; and how many heads an output count
//! asks for.

use nonos_gfx_wire::body::state::{LOST, SCANNING_OUT};
use nonos_gfx_wire::State;

use crate::display::head::Head;
use crate::display::head_rule::{head_slot, head_step, heads_missing, heads_wanted, HeadStep};
use crate::sw_blitter::Surface;

fn head(stale: bool) -> Head {
    let screen = Surface::default();
    Head {
        output: 1,
        serial: 0x0028_0002,
        handle: 9,
        screen,
        stale,
        owed: false,
        cursor_offset: 0,
        shaped: false,
    }
}

fn state(serial: u32, flags: u32) -> Option<State> {
    Some(State { serial, flags, ..State::default() })
}

#[test]
fn a_head_still_showing_its_primary_is_kept() {
    assert_eq!(head_step(&head(false), state(0x0028_0002, SCANNING_OUT)), HeadStep::Keep);
    assert_eq!(head_step(&head(false), None), HeadStep::Keep, "no answer this time");
}

#[test]
fn a_new_serial_or_a_stale_flush_asks_for_the_primary_again() {
    let s = state(0x0028_0003, SCANNING_OUT);
    assert_eq!(head_step(&head(false), s), HeadStep::Resurface);
    assert_eq!(head_step(&head(true), state(0x0028_0002, SCANNING_OUT)), HeadStep::Resurface);
    assert_eq!(head_step(&head(true), None), HeadStep::Resurface);
}

#[test]
fn a_lost_head_is_repainted() {
    let s = state(0x0028_0002, SCANNING_OUT | LOST);
    assert_eq!(head_step(&head(false), s), HeadStep::Repaint);
}

#[test]
fn every_output_past_the_first_is_a_head_up_to_the_servers_four() {
    assert_eq!((heads_wanted(0), heads_wanted(1), heads_wanted(2)), (0, 0, 1));
    assert_eq!((heads_wanted(4), heads_wanted(9)), (3, 3));
}

/// A FLUSH a mirrored or extended output refused for another reason than a
/// stale primary (a flip that timed out, a busy driver) was forgotten: a
/// window dragged across it stayed drawn at its old place there until other
/// damage covered it. The next poll sends that output the whole screen.
#[test]
fn a_head_whose_flush_was_refused_is_sent_the_screen_again() {
    let owed = Head { owed: true, ..head(false) };
    assert_eq!(head_step(&owed, state(0x0028_0002, SCANNING_OUT)), HeadStep::Resend);
    assert_eq!(head_step(&owed, None), HeadStep::Resend, "owed though the poll had no answer");
    let s = state(0x0028_0002, SCANNING_OUT | LOST);
    assert_eq!(head_step(&owed, s), HeadStep::Repaint, "lost: repainted, which pays it too");
    let stale = Head { owed: true, ..head(true) };
    assert_eq!(head_step(&stale, None), HeadStep::Resurface, "a new primary first");
}

#[test]
fn every_refusal_of_a_head_is_left_to_the_poll() {
    const MIRROR: &str = include_str!("../../../../compositor/src/frame_pacer/mirror.rs");
    const EXTEND: &str = include_str!("../../../../compositor/src/frame_pacer/extend.rs");
    const POLL: &str = include_str!("../../../../compositor/src/setup/poll_heads.rs");
    for src in [MIRROR, EXTEND] {
        assert!(src.contains("Err(ESTALE) => h.stale = true,"));
        assert!(src.contains("Err(_) => h.owed = true,"));
    }
    assert!(POLL.contains("HeadStep::Resend => {\n                ctx.heads[i].owed = false;"));
    assert!(POLL.contains("ctx.damage.mark_full(ctx.width, ctx.height);"));
}

/// A mirrored output that would not answer SURFACE used to have every head
/// let go, primed again and the whole screen marked on each 500 ms poll,
/// while nothing on screen moved. Only the outputs not held are asked
/// again now, and a poll where none of them answers repaints nothing.
#[test]
fn a_poll_asks_only_for_the_heads_not_held() {
    let missing = |outputs, have: &[u16]| heads_missing(outputs, have).collect::<Vec<_>>();
    assert_eq!(missing(1, &[]), Vec::<u16>::new(), "one output has no mirror");
    assert_eq!(missing(3, &[]), vec![1, 2]);
    assert_eq!(missing(3, &[1]), vec![2], "the head shown is not asked again");
    assert_eq!(missing(4, &[1, 3]), vec![2]);
    assert_eq!(missing(9, &[1, 2, 3]), Vec::<u16>::new(), "four outputs at most");
    assert_eq!(missing(0, &[]), Vec::<u16>::new());
}

#[test]
fn a_head_that_answers_late_takes_its_place_in_output_order() {
    assert_eq!(head_slot(2, &[1, 3]), 1);
    assert_eq!(head_slot(1, &[2, 3]), 0);
    assert_eq!(head_slot(3, &[1, 2]), 2);
    assert_eq!(head_slot(1, &[]), 0);
}

#[test]
fn a_head_still_missing_repaints_nothing_and_keeps_the_others() {
    const POLL: &str = include_str!("../../../../compositor/src/setup/poll_heads.rs");
    let at = |what: &str| POLL.find(what).expect(what);
    let missing = &POLL[at("fn prime_missing")..at("fn outputs_of")];
    assert!(!missing.contains("release_heads"), "the heads shown are kept");
    assert!(!missing.contains("mark_full"), "marked only through shown_anew");
    assert!(missing.contains("if added {\n        shown_anew(ctx, had);"));
    let poll = &POLL[at("pub fn poll_heads")..at("let mut what = None;")];
    assert_eq!(poll.matches("release_heads").count(), 1, "let go only when the count moved");
    assert!(poll.contains("if outputs != link.outputs {"));
}
