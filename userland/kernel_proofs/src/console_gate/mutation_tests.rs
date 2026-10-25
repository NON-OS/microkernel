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

//! Each rule undone in the model, one at a time from the kernel's gate:
//! the search finds what it breaks. The control comes first: a draw that
//! checks only before its count.

use super::actors::World;
use super::events::{Actor, Event};
use super::gate::{Flag, Gate, Recheck};
use super::outcome::{Broken, Failure};
use super::search::search;

fn broken(g: Gate, ignore: &[Broken]) -> Failure {
    search(&g, World::new(&g, 2, 1, true), ignore).expect_err("an undone rule went unseen")
}

#[test]
fn a_draw_checked_only_before_its_count_meets_a_quiet_takeover() {
    let g = Gate { recheck: Recheck::Absent, ..Gate::KERNEL };
    let f = search(&g, World::new(&g, 1, 1, false), &[]).expect_err("the control found nothing");
    assert_eq!(f.broken, Broken::DrawWhileQuiet, "{:?}", f.trace);
    let (t, last) = (&f.trace, f.trace.len() - 1);
    let (d, counted) = t[last];
    assert_eq!(counted, Event::Count, "{t:?}");
    let checked = (0..last).rev().find(|&i| t[i].0 == d).expect("its check");
    assert!(matches!(t[checked].1, Event::Load { again: false, stop: false, .. }), "{t:?}");
    let at = |e| t.iter().position(|s| matches!(s.0, Actor::Taker(_)) && s.1 == e);
    let (stop, quiet) = (at(Event::Stop).expect("a stop"), at(Event::Quiet).expect("quiet"));
    assert!(checked < stop && stop < quiet && quiet < last, "{t:?}");
}

#[test]
fn each_undone_rule_is_found() {
    let p = Gate::KERNEL;
    let cases = [
        (Gate { recheck: Recheck::HeldOnly, ..p }, Broken::DrawWhileQuiet),
        (Gate { waits: false, ..p }, Broken::QuietMidDraw),
        (Gate { counts_first: false, ..p }, Broken::DrawWhileQuiet),
        (Gate { ends: false, ..p }, Broken::NeverBack),
        (Gate { sets_gone: false, ..p }, Broken::BackAfterReady),
        (Gate { fence_first: false, ..p }, Broken::QuietMidDraw),
        (Gate { backs_out: false, ..p }, Broken::Stuck),
        (Gate { reads: &[Flag::Held, Flag::Gone], ..p }, Broken::StoppedWrong),
        (Gate { ends_last: false, ..p }, Broken::DrawWhileQuiet),
    ];
    for (g, want) in cases {
        let f = broken(g, &[]);
        assert_eq!(f.broken, want, "{g:?}: {:?}", f.trace);
    }
}

#[test]
fn stopped_without_takers_lets_a_draw_run_while_a_takeover_believes_quiet() {
    let g = Gate { reads: &[Flag::Held, Flag::Gone], ..Gate::KERNEL };
    assert_eq!(broken(g, &[Broken::StoppedWrong]).broken, Broken::DrawWhileQuiet);
}
