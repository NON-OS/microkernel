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

//! Step orders the model's gate holds as switches. A takeover ends after the
//! admit that latches GONE, as claim.rs calls end_takeover straight after
//! admit; ended before it, a draw gets past the latch. And the rule on the
//! claim itself, not on the flag: GONE loaded before TAKERS, or a GONE never
//! stored, lets a claim be made ready while a draw is under way.

use super::actors::World;
use super::events::{Actor, Event};
use super::gate::{Flag, Gate};
use super::outcome::Broken;
use super::search::search;

/// The first rule broken from `w`, `ignore` aside, and the steps to it.
fn first(g: &Gate, w: World, ignore: &[Broken]) -> (Broken, Vec<(Actor, Event)>) {
    let f = search(g, w, ignore).expect_err("every rule held");
    (f.broken, f.trace)
}

fn again(flag: Flag) -> Event {
    Event::Load { flag, again: true, stop: false }
}

#[test]
fn a_takeover_ended_before_its_admit_lets_a_draw_past_the_latch() {
    let g = Gate { ends_last: false, ..Gate::KERNEL };
    let w = World::new(&g, 1, 1, false);
    let (broken, t) = first(&g, w, &[]);
    assert_eq!(broken, Broken::DrawWhileQuiet, "{t:?}");
    let (d, last) = *t.last().expect("a step");
    assert_eq!(last, again(Flag::Gone), "{t:?}");
    let end = t.iter().position(|s| s.1 == Event::End { ready: true }).expect("its end");
    let count = t.iter().rposition(|s| s.0 == d && s.1 == Event::Count).expect("a count");
    assert!(count < end && !t.iter().any(|s| s.1 == Event::Latch), "{t:?}");
    assert_eq!(first(&g, w, &[Broken::DrawWhileQuiet]).0, Broken::DrawAfterGone);
    let flags = [Broken::DrawWhileQuiet, Broken::DrawAfterGone];
    assert_eq!(first(&g, w, &flags).0, Broken::ReadyMidDraw);
}

#[test]
fn gone_loaded_first_makes_a_claim_ready_while_a_draw_is_under_way() {
    let g = Gate::GONE_FIRST;
    let (broken, t) = first(&g, World::new(&g, 2, 1, true), &[Broken::DrawAfterGone]);
    assert_eq!(broken, Broken::ReadyMidDraw, "{t:?}");
    let (d, last) = *t.last().expect("a step");
    assert_eq!(last, again(Flag::Takers), "{t:?}");
    let gone = t.iter().rposition(|s| *s == (d, again(Flag::Gone))).expect("its GONE load");
    let latch = t.iter().position(|s| s.1 == Event::Latch).expect("latched");
    let end = t.iter().position(|s| s.1 == Event::End { ready: true }).expect("ended");
    assert!(gone < latch && latch < end, "{t:?}");
}

#[test]
fn with_gone_never_stored_a_ready_claim_also_meets_a_draw_under_way() {
    let g = Gate { sets_gone: false, ..Gate::KERNEL };
    let (broken, t) = first(&g, World::new(&g, 2, 1, true), &[Broken::BackAfterReady]);
    assert_eq!(broken, Broken::ReadyMidDraw, "{t:?}");
}
