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

//! Interrupt reports that land while another wait reads the ring are kept
//! for their own polls, every one of them: the list used to hold eight
//! and give up the oldest, which left that endpoint armed for good and its
//! device silent. A place is refused, before anything is armed, only when
//! every place is taken. A disabled slot's are dropped (`server_tests`).

use super::events::{transfer, CC_SUCCESS};
use super::fixture::{Fixture, BULK_IN_DCI, INT_DCI, SLOT};
use super::fixture_slots::interrupt_slot;
use crate::controller::{bulk_transfer, poll_interrupt_in, IntrPoll};
use crate::error::XhciError;
use crate::rings::event::state::ARMED;

#[test]
fn reports_that_land_during_one_bulk_wait_each_reach_their_own_poll() {
    let mut fx = Fixture::new();
    let mut pipes = fx.bulk_pipes();
    let mut slots: Vec<_> = (10..19).map(|s| interrupt_slot(&fx, s, &[INT_DCI])).collect();
    for res in slots.iter_mut() {
        let got = poll_interrupt_in(fx.db(), fx.intr(), &mut fx.ring, res, INT_DCI, 8);
        assert!(matches!(got, Ok(IntrPoll::Pending)), "armed");
        let armed = res.interrupt[0].armed.expect("armed");
        fx.hc.post(transfer(armed, CC_SUCCESS, 0, res.slot_id, INT_DCI));
    }
    let at = pipes.in_ring.enqueue_phys();
    fx.hc.post(transfer(at, CC_SUCCESS, 0, SLOT, BULK_IN_DCI));
    let moved = bulk_transfer(fx.db(), fx.intr(), &mut fx.ring, &mut pipes, SLOT, true, 512);
    assert_eq!(moved, Ok(512));
    for res in slots.iter_mut() {
        let got = poll_interrupt_in(fx.db(), fx.intr(), &mut fx.ring, res, INT_DCI, 8);
        assert!(matches!(got, Ok(IntrPoll::Complete(8))), "slot {}", res.slot_id);
    }
}

#[test]
fn arming_past_every_place_is_refused_before_anything_is_queued() {
    let mut fx = Fixture::new();
    let dcis = [3, 5, 7, 9];
    // Every place taken: four endpoints on as many slots as places allow.
    let full = (ARMED / dcis.len()) as u8;
    let mut slots: Vec<_> = (1..=full).map(|s| interrupt_slot(&fx, s, &dcis)).collect();
    for res in slots.iter_mut() {
        for dci in dcis {
            let got = poll_interrupt_in(fx.db(), fx.intr(), &mut fx.ring, res, dci, 8);
            assert!(matches!(got, Ok(IntrPoll::Pending)));
        }
    }
    let mut one_more = interrupt_slot(&fx, full + 1, &[INT_DCI]);
    let before = one_more.interrupt[0].ring.enqueue_phys();
    let got = poll_interrupt_in(fx.db(), fx.intr(), &mut fx.ring, &mut one_more, INT_DCI, 8);
    assert!(matches!(got, Err(XhciError::TooManyArmed)));
    assert_eq!(one_more.interrupt[0].ring.enqueue_phys(), before, "nothing queued");
    assert_eq!(one_more.interrupt[0].armed, None);
}
