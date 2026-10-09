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

//! The dequeue side of the ring: an event is the driver's to read once its
//! cycle bit equals the consumer cycle state, and not before. At the end of
//! the segment the dequeue pointer goes back to its start and the consumer
//! cycle state flips (xHCI 1.2 section 4.9.4), so what the last lap left in
//! a slot reads as not yet written until the controller writes it again.

use super::events::{numbered, port_change, transfer, CC_SUCCESS};
use super::fixture::{Fixture, INT_DCI, SLOT, TRB};
use super::producer::is_poison;
use crate::controller::{drain_events, DRAIN_BATCH};

#[test]
fn an_event_still_being_written_is_not_read() {
    let mut fx = Fixture::new();
    fx.hc.scribble(port_change(1));
    assert!(!fx.ring.has_event(), "the cycle bit says the slot is not written yet");
    let before = fx.dequeue_bus();
    assert_eq!(drain_events(fx.intr(), &mut fx.ring).count, 0);
    assert_eq!(fx.dequeue_bus(), before, "nothing was consumed");
    fx.hc.post(port_change(2));
    assert!(fx.ring.has_event());
    let batch = drain_events(fx.intr(), &mut fx.ring);
    assert_eq!(batch.count, 1);
    assert_eq!(batch.trbs[0].get_pointer() >> 24, 2, "the posted event, not the scribble");
}

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

#[test]
fn a_lap_of_one_event_at_a_time_reads_each_once_in_order() {
    let mut fx = Fixture::new();
    let base = fx.hc.segment_bus().start;
    for n in 0..fx.hc.segment_trbs() as u64 {
        assert_eq!(fx.dequeue_bus(), base + n * TRB);
        fx.hc.post(numbered(n as u32));
        let batch = drain_events(fx.intr(), &mut fx.ring);
        assert_eq!(batch.count, 1, "event {n}");
        assert!(!is_poison(&batch.trbs[0]), "event {n} was read from past the segment");
        assert_eq!(batch.trbs[0].d1 as u64, n);
    }
    assert_eq!(fx.dequeue_bus(), base, "the dequeue pointer wrapped to the segment start");
}

#[test]
fn nothing_is_read_after_a_full_lap_until_the_controller_writes_again() {
    let mut fx = Fixture::new();
    let trbs = fx.hc.segment_trbs() as u32;
    for n in 0..trbs {
        fx.hc.post(numbered(n));
        assert_eq!(drain_events(fx.intr(), &mut fx.ring).count, 1);
    }
    assert!(!fx.ring.has_event(), "last lap's event in slot 0 is not a new one");
    assert_eq!(drain_events(fx.intr(), &mut fx.ring).count, 0);
    fx.hc.post(numbered(trbs));
    let batch = drain_events(fx.intr(), &mut fx.ring);
    assert_eq!(batch.count, 1, "exactly the one the controller wrote");
    assert_eq!(batch.trbs[0].d1, trbs);
    assert!(!fx.ring.has_event());
}

/*
 * The rule for any starting dequeue index. The index is the driver's own and
 * moves only by reading, so each start is reached by reading that many
 * events; three laps follow in batches of one, of seven and of a full ring,
 * and every read is checked for order, for the segment bounds and for an
 * event read twice or read before it was written.
 */
#[test]
fn the_dequeue_pointer_wraps_with_the_cycle_flipped_from_every_start() {
    for start in 0..64u32 {
        let mut fx = Fixture::new();
        let trbs = fx.hc.segment_trbs() as u32;
        let base = fx.hc.segment_bus().start;
        for n in 0..start {
            fx.hc.post(numbered(n));
            assert_eq!(drain_events(fx.intr(), &mut fx.ring).count, 1);
        }
        let mut next = start;
        let mut read = start;
        for batch_size in [1, 7, trbs - 1] {
            let end = next + 3 * trbs;
            while next < end {
                let take = batch_size.min(end - next);
                for _ in 0..take {
                    fx.hc.post(numbered(next));
                    next += 1;
                }
                while fx.ring.has_event() {
                    let batch = drain_events(fx.intr(), &mut fx.ring);
                    for t in &batch.trbs[..batch.count] {
                        assert!(!is_poison(t), "start {start}: read past the segment");
                        assert_eq!(t.d1, read, "start {start}: out of order or read twice");
                        read += 1;
                    }
                }
                assert_eq!(read, next, "start {start}: an event was left unread");
                let at = fx.dequeue_bus();
                assert_eq!(at, base + (next % trbs) as u64 * TRB, "start {start}");
                assert_eq!(fx.hc.pending(), 0, "start {start}: ERDP lags the reads");
            }
        }
    }
}
