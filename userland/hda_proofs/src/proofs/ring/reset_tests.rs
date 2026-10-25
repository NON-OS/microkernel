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

//! A ring reset in place: the engines stopped and started with both CORB
//! pointers at 0, so nothing posted before the reset is fetched again.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use super::{link_on, ANSWER, TO_0};
use crate::constants::{
    CORBCTL, CORBCTL_RUN, CORBWP, RIRBCTL, RIRBCTL_DMAEN, RIRBCTL_RINTCTL, RIRBWP, RIRBWP_RST,
};
use crate::error::HdaError;
use crate::model::{fetch, live, rings, window};

#[test]
fn a_wedged_ring_is_reset_and_the_verb_answered_once() {
    let bar = window();
    let (corb, rirb) = rings(ANSWER);
    let read = Arc::new(Mutex::new(Vec::new()));
    // Fetches nothing until the driver resets RIRBWP, then reads the ring.
    let stuck = Arc::new(AtomicBool::new(true));
    let (c, r, s) = (Arc::clone(&corb), Arc::clone(&read), Arc::clone(&stuck));
    let _ring = live(&bar, move |b| {
        if b.wrote16(RIRBWP as usize) & RIRBWP_RST != 0 {
            s.store(false, Ordering::Release);
        }
        if !s.load(Ordering::Acquire) {
            r.lock().unwrap().extend(fetch(b, &c));
        }
        std::thread::yield_now();
    });
    let mut link = link_on(&bar, corb.base(), rirb.base());
    assert_eq!(link.send(TO_0), Err(HdaError::VerbTimeout), "one unanswered verb");
    assert_eq!(link.health.resets, 0, "one miss is not a reset");
    assert_eq!(link.send(TO_0), Ok(ANSWER), "the second is reset and answered");
    assert!(!stuck.load(Ordering::Acquire));
    assert_eq!(bar.wrote16(CORBWP as usize), 1, "the retry was not posted from the reset pointer");
    assert_eq!((link.health.resets, link.health.unfetched, link.health.immediate), (1, 0, false));
    assert_eq!(bar.wrote8(CORBCTL as usize) & CORBCTL_RUN, CORBCTL_RUN, "the CORB runs again");
    assert_eq!(bar.wrote8(RIRBCTL as usize), RIRBCTL_DMAEN | RIRBCTL_RINTCTL, "and the RIRB");
    assert_eq!(link.send(TO_0), Ok(ANSWER), "and goes on answering");
    // Neither verb from before the reset; the retry and the next, once each.
    assert_eq!(*read.lock().unwrap(), [TO_0, TO_0], "commands fetched again after the reset");
}
