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

//! The packet queue between the parser and the pump: it counts what it
//! holds across the wrap, and when full it drops new packets and counts them
//! rather than overwrite what is waiting.

use crate::mouse::event::MouseEvent;
use crate::mouse::parser::MouseParser;
use crate::mouse::ring::{MouseRing, MOUSE_RING_CAPACITY};

#[test]
fn the_queue_counts_what_it_holds_across_the_wrap() {
    let mut ring = MouseRing::new();
    let ev = MouseEvent { dx: 1, dy: 1, dz: 0, buttons: 0, flags: 0 };
    for round in 0..300 {
        (0..5).for_each(|_| ring.push(ev));
        assert_eq!(ring.queued(), 5, "round {round}");
        assert_eq!(core::iter::from_fn(|| ring.pop()).count(), 5);
    }
}

#[test]
fn a_full_queue_drops_new_packets_and_counts_them() {
    let mut p = MouseParser::new(false);
    let mut ring = MouseRing::new();
    let total = MOUSE_RING_CAPACITY + 10;
    for _ in 0..total {
        for b in [0x08u8, 0x01, 0x01] {
            p.absorb(b, 0, &mut ring);
        }
    }
    assert_eq!(ring.events_seen as usize, total);
    assert_eq!(ring.queued(), MOUSE_RING_CAPACITY - 1, "one slot always stays empty");
    assert_eq!(ring.events_dropped as usize, total - (MOUSE_RING_CAPACITY - 1));
}
