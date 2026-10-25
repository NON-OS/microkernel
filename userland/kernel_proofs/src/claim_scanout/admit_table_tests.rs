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

//! Whether a claim may be made ready beside the boot framebuffer (L79):
//! at_admit's whole table, every cell's answer written out by hand from the
//! design's reasons, one row per (is bootfb, scanout taken, covers) and one
//! letter per stage of bootfb's claim: '.' made ready, 'R' refused for the
//! rest of the boot, 'A' asked again.

use super::scanout_rule::at_admit;
use super::stage_rule::kept;
use super::types::{ClaimError, Stage};

const FB: [Option<Stage>; 6] = [
    None,
    Some(Stage::Making),
    Some(Stage::Ready),
    Some(Stage::Leaving),
    Some(Stage::Orphaned),
    Some(Stage::Exiting),
];

const TABLE: [(bool, bool, bool, &str, &str); 8] = [
    // Neither bootfb's claim nor a covering one: nothing is held back.
    (false, false, false, "......", "an unrelated claim"),
    (false, true, false, "......", "an unrelated claim after the takeover"),
    // A covering claim asks again while bootfb's claim stands or is being
    // released, as its pages may still show the scanout; else it goes in.
    (false, false, true, "..AA..", "a covering claim"),
    (false, true, true, "..AA..", "a covering claim after the takeover"),
    // bootfb's own claim before any takeover was latched: as before L79.
    (true, false, false, "......", "bootfb before the takeover"),
    // bootfb's claim once the scanout was taken: refused, whatever stands.
    (true, true, false, "RRRRRR", "bootfb after the takeover"),
    // bootfb never covers itself, as its record is published after covers
    // are found; were it to, the refusal comes first, the covering rule next.
    (true, false, true, "..AA..", "bootfb covering itself"),
    (true, true, true, "RRRRRR", "bootfb covering itself, taken"),
];

#[test]
fn every_cell_answers_as_the_design_reasons() {
    for (is_fb, taken, covers, row, who) in TABLE {
        assert_eq!(row.len(), FB.len(), "{who}");
        for (&fb, want) in FB.iter().zip(row.chars()) {
            let want = match want {
                '.' => Ok(()),
                'R' => Err(ClaimError::Refused),
                _ => Err(ClaimError::Resetting),
            };
            assert_eq!(at_admit(is_fb, taken, covers, fb), want, "{who}, bootfb {fb:?}");
        }
    }
}

#[test]
fn a_covering_claim_asks_again_exactly_while_bootfb_may_still_enter_a_grant() {
    for fb in FB.iter().flatten().copied() {
        let again = at_admit(false, false, true, Some(fb)) == Err(ClaimError::Resetting);
        assert_eq!(again, kept(fb), "{fb:?}");
    }
}
