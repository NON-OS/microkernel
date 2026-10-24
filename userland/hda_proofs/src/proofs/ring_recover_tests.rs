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
//! Verbs that stop being answered (C1): two the controller never fetched
//! reset the rings in place and the verb goes once more; three resets with
//! no answer between them, or rings that will not stop, and the driver
//! gives the rings up for the Immediate Command interface. A verb fetched
//! and unanswered is a silent codec, and the rings are left alone.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::constants::{
    CORBCTL, CORBCTL_RUN, CORBWP, ICII, ICIS, ICIS_ICB, ICIS_IRV, RIRBCTL, RIRBCTL_DMAEN,
    RIRBCTL_RINTCTL, RIRBSTS, RIRBSTS_OIS, RIRBWP, RIRBWP_RST,
};
use crate::controller::ring_recover::{MISSED_BEFORE_RESET, RESETS_BEFORE_IMMEDIATE};
use crate::controller::verb::Link;
use crate::error::HdaError;
use crate::model::{corb_engine, live, rings, window};
use crate::regs::Regs;

const ANSWER: u32 = 0x1234_5678;
/// Get Parameter 0 of codec 0's root, and of codec 1's.
const TO_0: u32 = 0x000f_0000;
const TO_1: u32 = 0x100f_0000;

fn link_on(bar: &nonos_devmodel::FakeBar, corb: u64, rirb: u64) -> Link {
    bar.present8(CORBCTL as usize, CORBCTL_RUN);
    bar.present8(RIRBCTL as usize, RIRBCTL_DMAEN | RIRBCTL_RINTCTL);
    Link::new(Regs::new(bar.base()), corb, rirb, 256)
}

#[test]
fn a_wedged_ring_is_reset_and_the_verb_answered() {
    let bar = window();
    let (corb, rirb) = rings(ANSWER);
    // Fetches nothing until the driver resets RIRBWP, then runs.
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
    assert_eq!(link.send(TO_0), Err(HdaError::VerbTimeout), "one unanswered verb");
    assert_eq!(link.health.resets, 0, "one miss is not a reset");
    assert_eq!(link.send(TO_0), Ok(ANSWER), "the second is reset and answered");
    assert!(!stuck.load(Ordering::Acquire));
    // CORBRP went back to 0: the retry goes in slot 1, or the controller
    // fetches every stale command between.
    assert_eq!(bar.wrote16(CORBWP as usize), 1, "the retry was not posted from the reset pointer");
    assert_eq!((link.health.resets, link.health.missed, link.health.immediate), (1, 0, false));
    assert_eq!(bar.wrote8(CORBCTL as usize) & CORBCTL_RUN, CORBCTL_RUN, "the CORB runs again");
    assert_eq!(bar.wrote8(RIRBCTL as usize), RIRBCTL_DMAEN | RIRBCTL_RINTCTL, "and the RIRB");
    assert_eq!(link.send(TO_0), Ok(ANSWER), "and goes on answering");
}

#[test]
fn a_dead_ring_is_given_up_after_its_resets_and_stopped() {
    let bar = window();
    let (corb, rirb) = rings(ANSWER);
    let mut link = link_on(&bar, corb.base(), rirb.base());
    let sends = MISSED_BEFORE_RESET as usize * (RESETS_BEFORE_IMMEDIATE as usize + 1);
    for _ in 0..sends {
        assert_eq!(link.send(TO_0), Err(HdaError::VerbTimeout));
    }
    assert!(link.health.immediate, "{sends} unanswered verbs and the rings still in use");
    assert_eq!(link.health.resets, RESETS_BEFORE_IMMEDIATE as u32);
    assert_eq!(bar.wrote8(CORBCTL as usize) & CORBCTL_RUN, 0, "the CORB left running");
    assert_eq!(bar.wrote8(RIRBCTL as usize) & RIRBCTL_DMAEN, 0, "the RIRB left running");
}

#[test]
fn one_silent_codec_never_takes_the_rings_from_one_that_answers() {
    let bar = window();
    let (corb, rirb) = rings(ANSWER);
    let _ring = live(&bar, corb_engine);
    let mut link = link_on(&bar, corb.base(), rirb.base());
    for _ in 0..(RESETS_BEFORE_IMMEDIATE as usize + 2) {
        for _ in 0..MISSED_BEFORE_RESET {
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
    for _ in 0..(MISSED_BEFORE_RESET as usize * (RESETS_BEFORE_IMMEDIATE as usize + 2)) {
        assert_eq!(link.send(TO_1), Err(HdaError::VerbTimeout));
    }
    assert_eq!((link.health.resets, link.health.missed, link.health.immediate), (0, 0, false));
}

#[test]
fn after_the_rings_are_given_up_verbs_go_one_at_a_time() {
    let bar = window();
    let (corb, rirb) = rings(ANSWER);
    // The Immediate interface alone answers: ICB taken, ICII filled, IRV set.
    let _immediate = live(&bar, |b| {
        if b.wrote16(ICIS as usize) & ICIS_ICB != 0 {
            b.present32(ICII as usize, 0xabcd_0001);
            b.present16(ICIS as usize, ICIS_IRV);
        }
        std::thread::yield_now();
    });
    let mut link = link_on(&bar, corb.base(), rirb.base());
    let sends = MISSED_BEFORE_RESET as usize * (RESETS_BEFORE_IMMEDIATE as usize + 1);
    let answers: Vec<_> = (0..sends).map(|_| link.send(TO_0)).collect();
    assert!(link.health.immediate);
    assert_eq!(answers.last(), Some(&Ok(0xabcd_0001)), "the giving up verb went immediately");
    assert_eq!(link.send(TO_0), Ok(0xabcd_0001), "and every one after");
}

#[test]
fn rings_that_will_not_stop_are_given_up_at_the_first_recovery() {
    let bar = window();
    let (corb, rirb) = rings(ANSWER);
    // An engine that shows its run bit whatever is written.
    let _ring = live(&bar, |b| {
        b.present8(CORBCTL as usize, CORBCTL_RUN);
        std::thread::yield_now();
    });
    let mut link = link_on(&bar, corb.base(), rirb.base());
    for _ in 0..MISSED_BEFORE_RESET {
        let _ = link.send(TO_0);
    }
    assert!(link.health.immediate, "a ring that would not stop was reset");
    assert_eq!(link.health.resets, 0);
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
