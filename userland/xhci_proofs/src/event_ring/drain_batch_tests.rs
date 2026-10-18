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

//! The idle drain reads a bounded batch of events and hands each back to
//! the controller, and stops at a transfer event, which is its waiter's.

use super::events::{port_change, transfer, CC_SUCCESS};
use super::fixture::{Fixture, INT_DCI, SLOT};
use super::producer::is_poison;
use crate::controller::drain_events;
use crate::controller::drain_events::DRAIN_BATCH;

#[test]
fn a_drain_reads_at_most_one_batch() {
    let mut fx = Fixture::new();
    for n in 0..40u8 {
        fx.hc.post(port_change(n + 1));
    }
    let first = drain_events(fx.intr(), &mut fx.ring);
    let second = drain_events(fx.intr(), &mut fx.ring);
    assert_eq!((first.count, second.count), (DRAIN_BATCH, 40 - DRAIN_BATCH));
    let read = first.trbs[..first.count].iter().chain(&second.trbs[..second.count]);
    for (n, t) in read.enumerate() {
        assert!(!is_poison(t), "event {n} was read from past the segment");
        assert_eq!(t.get_pointer() >> 24, n as u64 + 1, "in the order written");
    }
    assert_eq!(fx.hc.pending(), 0);
}

#[test]
fn a_drain_leaves_a_transfer_event_for_its_waiter() {
    let mut fx = Fixture::new();
    fx.hc.post(port_change(1));
    fx.hc.post(transfer(0x1000, CC_SUCCESS, 0, SLOT, INT_DCI));
    fx.hc.post(port_change(2));
    assert_eq!(drain_events(fx.intr(), &mut fx.ring).count, 1);
    assert!(fx.ring.has_event(), "the transfer event is still at the head");
    assert_eq!(fx.hc.pending(), 2);
}
