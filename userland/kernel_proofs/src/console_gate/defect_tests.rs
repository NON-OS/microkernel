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

//! The gate as first written. `stopped()` loaded GONE before TAKERS, and a
//! takeover stores GONE before it leaves TAKERS, so a draw whose second check
//! loaded GONE just before the latch and TAKERS just after the takeover ended
//! found neither, and wrote on the scanout the GPU driver's claim now holds.
//! Every other rule held. The kernel now loads TAKERS first (model_tests.rs).

use super::actors::World;
use super::events::Event;
use super::gate::{Flag, Gate};
use super::outcome::Broken;
use super::search::search;

#[test]
fn gone_loaded_first_lets_one_draw_past_the_latch() {
    let g = Gate::GONE_FIRST;
    let f = search(&g, World::new(&g, 2, 1, true), &[]).expect_err("no draw got past the latch");
    assert_eq!(f.broken, Broken::DrawAfterGone, "{:?}", f.trace);
    let (t, last) = (&f.trace, f.trace.len() - 1);
    let (d, passed) = t[last];
    assert_eq!(passed, Event::Load { flag: Flag::Takers, again: true, stop: false }, "{t:?}");
    let gone = (0..last).rev().find(|&i| t[i].0 == d).expect("the draw's load before");
    assert_eq!(t[gone].1, Event::Load { flag: Flag::Gone, again: true, stop: false }, "{t:?}");
    let between: Vec<Event> = t[gone + 1..last].iter().map(|s| s.1).collect();
    let latch = between.iter().position(|e| *e == Event::Latch).expect("latched between");
    assert!(between[latch..].contains(&Event::End { ready: true }), "{t:?}");
}

#[test]
fn every_rule_but_the_latch_holds_with_gone_loaded_first() {
    let g = Gate::GONE_FIRST;
    let latch = [Broken::DrawAfterGone, Broken::ReadyMidDraw];
    for (takers, states, runs) in [(1, 11702, 13978287542588618), (2, 62972, 5596983590912183056)] {
        let c = search(&g, World::new(&g, 2, takers, true), &latch);
        let c = c.unwrap_or_else(|f| panic!("{f:?}"));
        assert!(c.states >= states && c.runs >= runs, "{takers} takeovers: {c:?}");
    }
}
