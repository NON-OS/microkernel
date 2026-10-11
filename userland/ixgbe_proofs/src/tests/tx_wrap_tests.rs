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

//! The transmit ring across thousands of laps: bursts of posts, the part
//! writing DD back on each, and reclaim catching up every time.

use crate::constants::queue::TX_DESC_COUNT;
use crate::constants::tx_bits::TXD_STAT_DD;

use super::memory::Memory;

const N: u16 = TX_DESC_COUNT as u16;

#[test]
fn many_laps_of_post_complete_reclaim_wrap_cleanly() {
    let mut mem = Memory::new();
    let mut tx = mem.tx_ring();
    for n in 0..10_000u32 {
        let burst = (n % 7 + 1) as u16;
        let mut posted = Vec::new();
        for _ in 0..burst {
            assert!(!tx.full());
            posted.push(tx.post(60 + burst));
        }
        for idx in posted {
            assert!(idx < N);
            mem.tx[idx as usize].olinfo_status |= TXD_STAT_DD;
        }
        tx.reclaim();
        assert_eq!(tx.clean, tx.tail, "every written-back slot is reclaimed");
    }
}
