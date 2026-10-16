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

//! The dequeue rule of `dequeue_tests` (xHCI 1.2 section 4.9.4), from
//! every index the dequeue pointer can start at.

use super::events::numbered;
use super::fixture::{Fixture, TRB};
use super::producer::is_poison;
use crate::controller::drain_events;

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
