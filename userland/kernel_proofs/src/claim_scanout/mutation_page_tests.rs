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

//! The take-back's page steps undone one at a time (L79), each a knob on
//! the model, over starts where the model as built breaks nothing.

use super::knob::Knob;
use super::mutation_order_tests::undone;
use super::starts::Sib;
use super::world_check::{CLOBBER, OUTLIVED, TWO_WRITERS};
use super::world_names::names;

#[test]
fn the_release_before_the_repoint_lets_the_entry_go_under_live_pages() {
    // G1 alone: bootfb's entry gone while its pages still show the scanout.
    let broken = undone(Knob::ReleaseFirst, |c| !c.g2 && !c.doomed, 100_000);
    assert!(broken & OUTLIVED != 0, "{:?}", names(broken));
    // With G2: G2 finds no claim and is made ready beside those pages.
    let broken = undone(Knob::ReleaseFirst, |c| c.g2 && !c.doomed && c.sib == Sib::Idle, 100_000);
    assert!(broken & TWO_WRITERS != 0, "{:?}", names(broken));
}

#[test]
fn an_unmap_outside_its_claim_lets_the_gpu_in_before_the_pages_go() {
    let broken = undone(Knob::UnmapOutside, |c| !c.g2 && c.sib == Sib::Unmap, 50_000);
    let wanted = TWO_WRITERS | OUTLIVED;
    assert_eq!(broken & wanted, wanted, "{:?}", names(broken));
}

#[test]
fn a_repoint_that_does_not_ask_what_a_page_shows_clobbers_another_mapping() {
    let broken = undone(Knob::NoTranslateCheck, |c| !c.g2 && c.sib == Sib::Stray, 50_000);
    assert!(broken & CLOBBER != 0, "{:?}", names(broken));
}
