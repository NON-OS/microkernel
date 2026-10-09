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
//! Giving up on a controller that never answers.
//!
//! The BAR decodes, the registers accept writes, and the DMA engine does
//! nothing. Every wait has to end, and in time rather than in spins.

use std::time::Instant;

use crate::constants::CORBWP;
use crate::controller::codec::walk::walk;
use crate::error::HdaError;
use crate::model::{rings, window};
use crate::proofs::verb_tests::{link_over, CMD, RESPONSE};

#[test]
fn a_controller_that_never_answers_is_given_up_on_rather_than_waited_on() {
    let bar = window();
    let (corb, rirb) = rings(RESPONSE);
    let mut link = link_over(&bar, corb.base(), rirb.base());
    let t = Instant::now();
    assert_eq!(link.send(CMD), Err(HdaError::VerbTimeout), "an unanswered verb must refuse");
    assert!(t.elapsed().as_millis() < 1000, "the wait was not bounded in time");
}

#[test]
fn after_an_unanswered_verb_the_shadow_pointer_agrees_with_the_controller() {
    /*
     * The command did move CORBWP. A shadow left behind put the next command
     * in the same slot under the same CORBWP, which the controller reads as
     * nothing new: after one slow answer every later verb timed out.
     */
    let bar = window();
    let (corb, rirb) = rings(RESPONSE);
    let mut link = link_over(&bar, corb.base(), rirb.base());
    let _ = link.send(CMD);
    assert_eq!(bar.wrote16(CORBWP as usize), 1);
    let _ = link.send(CMD + 1);
    assert_eq!(bar.wrote16(CORBWP as usize), 2, "the second command was not posted past the first");
}

#[test]
fn the_codec_walk_ends_when_the_first_question_goes_unanswered() {
    let bar = window();
    let (corb, rirb) = rings(RESPONSE);
    let mut link = link_over(&bar, corb.base(), rirb.base());
    assert!(walk(&mut link, 0).is_err(), "a silent codec must not yield a description");
}
