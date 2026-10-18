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

//! The PCM queue on its own: what it takes, what it hands back, and that a
//! short queue never leaves a previous period's samples in the DMA ring.

use crate::audio::PcmQueue;

#[test]
fn a_short_pop_fills_the_rest_of_the_period_with_silence() {
    /*
     * The refill copies a whole period into the ring. Whatever the queue
     * cannot supply must be silence, not the bytes the ring held from the
     * period before, which would replay as a stutter.
     */
    let mut q = PcmQueue::new();
    assert_eq!(q.push(&[1, 2, 3, 4]), 4);
    let mut out = [9u8; 6];
    assert_eq!(q.pop_into(&mut out), 4);
    assert_eq!(out, [1, 2, 3, 4, 0, 0], "the tail of the period kept old bytes");
    let mut again = [9u8; 3];
    assert_eq!(q.pop_into(&mut again), 0, "an emptied queue gave more");
    assert_eq!(again, [0, 0, 0], "an empty queue left old bytes in the period");
}

#[test]
fn a_push_past_the_room_left_takes_only_what_fits() {
    let mut q = PcmQueue::new();
    let big = vec![7u8; 0x1_0000 + 100];
    let took = q.push(&big);
    assert!(took < big.len(), "the queue took more than its 64 KiB");
    assert!(q.is_full_for(1), "a full queue reported room");
    q.clear();
    assert!(!q.is_full_for(took), "a cleared queue reported no room");
    let mut out = [9u8; 2];
    assert_eq!(q.pop_into(&mut out), 0, "a cleared queue still held samples");
}
