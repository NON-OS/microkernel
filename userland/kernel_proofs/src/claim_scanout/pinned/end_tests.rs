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

//! The model's ends held to the console it frees (L79). The first end checks
//! named GONE and the takeovers' count but not HELD, so a hold left set as
//! bootfb's entry went would have passed while the console stayed dark; now
//! no end without a covering claim made ready may leave anything stopping
//! the kernel's draws, and undoing the hold's clearing shows the check bites.
//! And every start's own search reaches worlds and ends, against a floor
//! each, where the first floors were on totals a dead start could hide in.

use super::super::knob::Knob;
use super::super::model_tests::built;
use super::super::mutation_order_tests::breaks;
use super::super::starts::Cfg;
use super::super::world_check::STOPPED_LEFT;

/// Worlds and ends each start's search reaches at least: as built the least
/// are 2,163 and 10 with G1's holder alive, 4,066 and 8 with it exiting.
const LIVE: (usize, usize) = (2_000, 8);
const DOOMED: (usize, usize) = (3_500, 6);

#[test]
fn a_hold_left_set_as_bootfbs_entry_goes_leaves_the_console_stopped() {
    breaks(Knob::HoldKept, |c: &Cfg| !c.g2 && !c.again, 20_000, STOPPED_LEFT);
}

#[test]
fn every_start_searches_worlds_and_ends_of_its_own() {
    for (c, f) in built() {
        let floor = if c.doomed { DOOMED } else { LIVE };
        assert!(f.worlds >= floor.0 && f.ends >= floor.1, "{c:?}: {f:?}");
        // A live start makes the GPU ready in most of its ends (9 at least).
        assert!(c.doomed || f.ready >= 6, "{c:?}: the GPU too seldom made ready");
    }
}
