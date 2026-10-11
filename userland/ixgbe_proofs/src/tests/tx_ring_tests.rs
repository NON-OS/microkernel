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

//! The transmit ring: what a post writes, when the ring is full, and that
//! reclaim follows the part's DD write-backs in order. The wrap is in
//! tx_wrap_tests.

use crate::constants::queue::{TX_BUFFER_LEN, TX_DESC_COUNT};
use crate::constants::tx_bits::TXD_STAT_DD;
use crate::queue::tx_encode::{cmd_type_len, olinfo_status};

use super::memory::{Memory, TX_BUF_PHYS};

const N: u16 = TX_DESC_COUNT as u16;

#[test]
fn a_post_writes_the_advanced_data_descriptor_for_its_slot() {
    let mut mem = Memory::new();
    let mut tx = mem.tx_ring();
    for i in 0..3u16 {
        assert_eq!(tx.post(100 + i), i);
    }
    let d = mem.tx[2];
    assert_eq!(d.buffer_addr, TX_BUF_PHYS + 2 * TX_BUFFER_LEN as u64);
    assert_eq!(d.cmd_type_len, cmd_type_len(102));
    assert_eq!(d.olinfo_status, olinfo_status(102));
    assert_eq!(tx.tail, 3);
}

#[test]
fn the_ring_is_full_one_short_and_reclaim_frees_only_done_slots_in_order() {
    let mut mem = Memory::new();
    let mut tx = mem.tx_ring();
    for _ in 0..N - 1 {
        assert!(!tx.full());
        tx.post(60);
    }
    assert!(tx.full(), "TDT may never be moved onto TDH");
    tx.reclaim();
    assert_eq!(tx.clean, 0, "nothing written back, nothing reclaimed");
    mem.tx[1].olinfo_status |= TXD_STAT_DD;
    tx.reclaim();
    assert_eq!(tx.clean, 0, "slot 0 is still the part's, so slot 1 waits");
    for i in 0..5 {
        mem.tx[i].olinfo_status |= TXD_STAT_DD;
    }
    tx.reclaim();
    assert_eq!(tx.clean, 5);
    assert!(!tx.full());
}
