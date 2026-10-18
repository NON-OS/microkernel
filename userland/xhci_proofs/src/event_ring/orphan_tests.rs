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

//! Command completions left behind by an abort or a wait that gave up. No
//! wait ever takes them, so the drain and the interrupt poll take them off
//! instead of stopping in front of them.

use super::events::{command, port_change, transfer, CC_SUCCESS};
use super::fixture::{Fixture, INT_DCI, SLOT, TRB};
use crate::controller::{drain_events, poll_interrupt_in, IntrPoll, DRAIN_BATCH};

#[test]
fn the_interrupt_poll_takes_an_orphan_off_and_completes_the_report_behind_it() {
    let mut fx = Fixture::new();
    let mut res = fx.hid_slot();
    let issued = res.interrupt[0].ring.enqueue_phys();
    fx.hc.post(command(0x1000, CC_SUCCESS, SLOT));
    fx.hc.post(command(0x1000 + TRB, CC_SUCCESS, SLOT));
    fx.hc.post(transfer(issued, CC_SUCCESS, 3, SLOT, INT_DCI));
    let got = poll_interrupt_in(fx.db(), fx.intr(), &mut fx.ring, &mut res, INT_DCI, 8);
    assert!(matches!(got, Ok(IntrPoll::Complete(5))), "the report was not held up");
    assert_eq!(fx.ring.orphans(), 2);
    assert_eq!(fx.hc.pending(), 0);
}

#[test]
fn the_drain_takes_orphans_off_and_keeps_the_events_behind_them() {
    let mut fx = Fixture::new();
    fx.hc.post(command(0x1000, CC_SUCCESS, SLOT));
    fx.hc.post(port_change(1));
    fx.hc.post(command(0x1000 + TRB, CC_SUCCESS, SLOT));
    fx.hc.post(port_change(2));
    let batch = drain_events(fx.intr(), &mut fx.ring);
    assert_eq!(batch.count, 2, "both port changes, neither orphan");
    assert_eq!(fx.ring.orphans(), 2);
    assert_eq!(fx.hc.pending(), 0);
}

#[test]
fn a_ring_of_orphans_is_drained_a_batch_at_a_time() {
    let mut fx = Fixture::new();
    for i in 0..DRAIN_BATCH as u64 + 5 {
        fx.hc.post(command(0x1000 + i * TRB, CC_SUCCESS, SLOT));
    }
    let first = drain_events(fx.intr(), &mut fx.ring);
    assert_eq!(first.count, 0);
    assert_eq!(fx.ring.orphans(), DRAIN_BATCH as u64, "an orphan counts against the batch");
    assert_eq!(fx.hc.pending(), 5);
    let _ = drain_events(fx.intr(), &mut fx.ring);
    assert_eq!(fx.hc.pending(), 0);
}

#[test]
fn the_drain_still_stops_at_a_transfer_event() {
    let mut fx = Fixture::new();
    fx.hc.post(command(0x1000, CC_SUCCESS, SLOT));
    fx.hc.post(transfer(0x2000, CC_SUCCESS, 0, SLOT, INT_DCI));
    let batch = drain_events(fx.intr(), &mut fx.ring);
    assert_eq!(batch.count, 0);
    assert_eq!(fx.hc.pending(), 1, "left for its endpoint's poll");
}
