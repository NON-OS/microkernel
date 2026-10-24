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

//! The rings given up: after resets that stopped helping, or at the first
//! recovery when an engine will not stop, and verbs one at a time after.

use super::{link_on, ANSWER, TO_0};
use crate::clock::on_poll;
use crate::constants::{CORBCTL, CORBCTL_RUN, RIRBCTL, RIRBCTL_DMAEN};
use crate::controller::ring_recover::{RESETS_BEFORE_IMMEDIATE, UNFETCHED_BEFORE_RESET};
use crate::error::HdaError;
use crate::model::{rings, window, Immediate};

/// Sends that run a dead ring through every reset to the giving up.
const TO_GIVE_UP: usize = UNFETCHED_BEFORE_RESET as usize * (RESETS_BEFORE_IMMEDIATE as usize + 1);

#[test]
fn a_dead_ring_is_given_up_after_its_resets_and_stopped() {
    let bar = window();
    let (corb, rirb) = rings(ANSWER);
    let mut link = link_on(&bar, corb.base(), rirb.base());
    for _ in 0..TO_GIVE_UP {
        assert_eq!(link.send(TO_0), Err(HdaError::VerbTimeout));
    }
    assert!(link.health.immediate, "{TO_GIVE_UP} unanswered verbs and the rings still in use");
    assert_eq!(link.health.resets, RESETS_BEFORE_IMMEDIATE as u32);
    assert_eq!(bar.wrote8(CORBCTL as usize) & CORBCTL_RUN, 0, "the CORB left running");
    assert_eq!(bar.wrote8(RIRBCTL as usize) & RIRBCTL_DMAEN, 0, "the RIRB left running");
}

#[test]
fn after_the_rings_are_given_up_verbs_go_one_at_a_time() {
    let bar = window();
    let (corb, rirb) = rings(ANSWER);
    // The Immediate interface alone answers.
    let mut immediate = Immediate::new(&bar, 0xabcd_0001, None);
    let _model = on_poll(move || immediate.step());
    let mut link = link_on(&bar, corb.base(), rirb.base());
    let answers: Vec<_> = (0..TO_GIVE_UP).map(|_| link.send(TO_0)).collect();
    assert!(link.health.immediate);
    assert_eq!(answers.last(), Some(&Ok(0xabcd_0001)), "the giving up verb went immediately");
    assert_eq!(link.send(TO_0), Ok(0xabcd_0001), "and every one after");
    assert_eq!(link.send(TO_0), Ok(0xabcd_0001), "with IRV cleared between them");
}

#[test]
fn rings_that_will_not_stop_are_given_up_at_the_first_recovery() {
    let bar = window();
    let (corb, rirb) = rings(ANSWER);
    // An engine that shows its run bit whatever is written, on every poll,
    // so the driver never reads back the clear it wrote.
    let b = std::sync::Arc::clone(&bar);
    let _model = on_poll(move || b.present8(CORBCTL as usize, CORBCTL_RUN));
    let mut link = link_on(&bar, corb.base(), rirb.base());
    for _ in 0..UNFETCHED_BEFORE_RESET {
        let _ = link.send(TO_0);
    }
    assert!(link.health.immediate, "a ring that would not stop was reset");
    assert_eq!(link.health.resets, 0);
}
