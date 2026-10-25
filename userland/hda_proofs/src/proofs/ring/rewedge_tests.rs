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

//! A ring that wedges again after every answer is reset each time and kept:
//! an answer between two resets shows the reset worked.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use super::{link_on, ANSWER, TO_0};
use crate::constants::{CORBWP, RIRBWP, RIRBWP_RST};
use crate::controller::ring_recover::RESETS_BEFORE_IMMEDIATE;
use crate::model::{corb_engine, live, rings, window};

#[test]
fn a_ring_that_wedges_again_after_each_answer_keeps_its_rings() {
    let bar = window();
    let (corb, rirb) = rings(ANSWER);
    let stuck = Arc::new(AtomicBool::new(true));
    let s = Arc::clone(&stuck);
    let _ring = live(&bar, move |b| {
        if b.wrote16(RIRBWP as usize) & RIRBWP_RST != 0 {
            s.store(false, Ordering::Release);
        }
        if !s.load(Ordering::Acquire) {
            corb_engine(b);
        }
    });
    let mut link = link_on(&bar, corb.base(), rirb.base());
    for round in 0..(RESETS_BEFORE_IMMEDIATE as u32 + 2) {
        stuck.store(true, Ordering::Release);
        // The driver's reset writes RIRBWP_RST; the model clears it as RIRBWP.
        bar.present16(RIRBWP as usize, bar.wrote16(CORBWP as usize) & 0xff);
        let _ = link.send(TO_0);
        assert_eq!(link.send(TO_0), Ok(ANSWER), "round {round}");
    }
    assert!(!link.health.immediate, "answers between the resets did not count");
    assert_eq!(link.health.resets, RESETS_BEFORE_IMMEDIATE as u32 + 2);
}
