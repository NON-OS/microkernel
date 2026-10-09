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

//! TX ring invariants against a part that completes descriptors at random:
//! a post fills exactly the tail slot and wraps, `reclaim` only passes
//! descriptors the part marked done and in order, `full` keeps one slot
//! empty, and no post ever lands on a slot the part still owns.

use super::xorshift;
use crate::constants::queue::{TX_BUFFER_LEN, TX_CMD_EOP, TX_CMD_IFCS, TX_CMD_RS, TX_DESC_COUNT};
use crate::queue::layout::TxDesc;
use crate::queue::TxRing;

const N: u16 = TX_DESC_COUNT as u16;

#[test]
fn post_fills_the_tail_slot_and_wraps() {
    let mut ring = [TxDesc::default(); TX_DESC_COUNT];
    let mut tx = TxRing::new(ring.as_mut_ptr() as u64, 0, 0x40_0000);
    for round in 0..(2 * N) {
        let idx = tx.post(64 + round);
        assert_eq!(idx, round % N);
        assert_eq!(tx.tail, (idx + 1) % N);
        let d = ring[idx as usize];
        assert_eq!((d.length, d.status), (64 + round, 0));
        assert_eq!(d.cmd, TX_CMD_EOP | TX_CMD_IFCS | TX_CMD_RS);
        assert_eq!(d.buffer_addr, 0x40_0000 + idx as u64 * TX_BUFFER_LEN as u64);
        ring[idx as usize].status = 1;
        tx.reclaim();
    }
}

#[test]
fn reclaim_and_full_never_hand_out_a_slot_the_part_owns() {
    for seed in 1..2_000u64 {
        let mut s = seed;
        let mut ring = [TxDesc::default(); TX_DESC_COUNT];
        let mut tx = TxRing::new(ring.as_mut_ptr() as u64, 0, 0);
        // The part completes in order; `done_upto` is its next descriptor.
        let mut done_upto = 0u16;
        for _ in 0..400 {
            if !xorshift(&mut s).is_multiple_of(3) {
                tx.reclaim();
                if tx.full() {
                    assert_eq!(owned(&tx), N - 1, "full is N-1 slots with the part");
                    continue;
                }
                tx.post(60);
            } else if done_upto != tx.tail {
                ring[done_upto as usize].status = 1;
                done_upto = (done_upto + 1) % N;
            }
            tx.reclaim();
            assert!(owned(&tx) < N, "one slot always stays empty");
            assert_eq!(tx.clean, done_upto, "reclaim passes every done slot and no other");
        }
    }
}

fn owned(tx: &TxRing) -> u16 {
    (tx.tail + N - tx.clean) % N
}
