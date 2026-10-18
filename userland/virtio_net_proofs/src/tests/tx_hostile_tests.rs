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

//! Completions a hostile device makes up: an id never posted, one returned
//! twice, one past the buffers, an index thrown far ahead. None of them
//! frees a buffer the device still has; a real one comes back wiped.

use super::harness::Dev;
use crate::constants::{RING_SLOTS, TX_BUFFER_LEN, TX_DESC_COUNT};
use crate::queue::TxError;

#[test]
fn only_a_buffer_posted_and_not_yet_returned_comes_back() {
    let dev = Dev::tx();
    let mut tx = dev.tx_queue();
    tx.post(&[1; 60]).unwrap();
    tx.post(&[2; 60]).unwrap();
    // Never posted, past the buffers, far past them, then 1 twice.
    for (n, id) in [6u32, 8, 0x1_0001, 1, 1].into_iter().enumerate() {
        dev.complete(n as u16, id, 0);
    }
    tx.reclaim();
    assert_eq!(tx.in_flight, 0b01, "buffer 0 is still the device's");
    assert_eq!(tx.free_slot(), Some(1));
}

#[test]
fn an_index_thrown_far_ahead_is_read_a_ring_at_a_time() {
    let dev = Dev::tx();
    let mut tx = dev.tx_queue();
    for _ in 0..TX_DESC_COUNT {
        tx.post(&[0; 60]).unwrap();
    }
    for n in 0..RING_SLOTS {
        dev.complete(n, 99, 0);
    }
    dev.complete(60_000, 99, 0);
    tx.reclaim();
    assert_eq!(tx.last_used, RING_SLOTS, "more than one ring's worth read in one call");
    assert_eq!(tx.post(&[0; 60]), Err(TxError::Full), "a made-up entry freed a buffer");
}

#[test]
fn a_returned_buffer_is_wiped() {
    let dev = Dev::tx();
    let mut tx = dev.tx_queue();
    tx.post(&[0x5A; 1000]).unwrap();
    dev.complete(0, 0, 0);
    tx.reclaim();
    assert!((0..TX_BUFFER_LEN as usize).all(|i| dev.bufs.get(i) == 0), "the frame stayed");
    assert_eq!(tx.in_flight, 0);
}
