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

//! The send consumer index the status block reports, believed only when it
//! names frames that are in flight, and the ring and its slots wrapping in
//! step.

use crate::constants::layout::TX_BUFFER_LEN;
use crate::constants::ring::TX_RING_SIZE;

use super::memory::Memory;

#[test]
fn a_hostile_consumer_index_is_not_believed() {
    let mut m = Memory::new();
    let mut t = m.tx_ring();
    t.init();
    for _ in 0..3 {
        t.post(60);
    }
    for hw in [TX_RING_SIZE as u16, 0xFFFF, 4, 200, 511] {
        t.reclaim(hw);
        assert_eq!((t.cons, t.in_flight()), (0, 3), "{hw}");
    }
    t.reclaim(3);
    assert_eq!(t.in_flight(), 0);
}

#[test]
fn slots_and_indexes_wrap_together() {
    let mut m = Memory::new();
    let mut t = m.tx_ring();
    t.init();
    let base = t.next_buffer_va();
    for _ in 0..TX_RING_SIZE + 5 {
        t.post(60);
        t.reclaim(t.prod);
    }
    assert_eq!(t.prod, 5);
    assert_eq!(t.next_buffer_va(), base + (5 * TX_BUFFER_LEN) as u64);
}
