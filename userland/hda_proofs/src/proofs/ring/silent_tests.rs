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

//! A codec that never answers is not a wedged ring: the controller fetched
//! its verbs, and the rings are kept.

use super::{link_on, ANSWER, TO_0, TO_1};
use crate::constants::{RIRBSTS, RIRBSTS_OIS};
use crate::controller::ring_recover::{RESETS_BEFORE_IMMEDIATE, UNFETCHED_BEFORE_RESET};
use crate::error::HdaError;
use crate::model::{corb_engine, live, rings, window};

#[test]
fn one_silent_codec_never_takes_the_rings_from_one_that_answers() {
    let bar = window();
    let (corb, rirb) = rings(ANSWER);
    let _ring = live(&bar, corb_engine);
    let mut link = link_on(&bar, corb.base(), rirb.base());
    for _ in 0..(RESETS_BEFORE_IMMEDIATE as usize + 2) {
        for _ in 0..UNFETCHED_BEFORE_RESET {
            assert_eq!(link.send(TO_1), Err(HdaError::VerbTimeout), "codec 1 is silent");
        }
        assert_eq!(link.send(TO_0), Ok(ANSWER), "codec 0 answers");
    }
    assert!(!link.health.immediate, "the rings were given up for one silent codec");
    assert_eq!(link.health.resets, 0, "working rings were reset for a silent codec");
}

#[test]
fn a_silent_codec_alone_on_the_wire_leaves_the_rings_alone() {
    let bar = window();
    let (corb, rirb) = rings(ANSWER);
    let _ring = live(&bar, corb_engine);
    let mut link = link_on(&bar, corb.base(), rirb.base());
    for _ in 0..(UNFETCHED_BEFORE_RESET as usize * (RESETS_BEFORE_IMMEDIATE as usize + 2)) {
        assert_eq!(link.send(TO_1), Err(HdaError::VerbTimeout));
    }
    assert_eq!((link.health.resets, link.health.unfetched, link.health.immediate), (0, 0, false));
}

#[test]
fn a_response_the_rirb_dropped_is_counted() {
    let bar = window();
    let (corb, rirb) = rings(ANSWER);
    let _ring = live(&bar, corb_engine);
    let mut link = link_on(&bar, corb.base(), rirb.base());
    bar.present8(RIRBSTS as usize, RIRBSTS_OIS);
    assert_eq!(link.send(TO_0), Ok(ANSWER));
    assert!(link.health.overruns >= 1, "an overrun went unseen");
}
