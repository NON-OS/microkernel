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

//! The transmit watchdog: a stall past the bound with the link up resets the
//! part and starts both rings over; link down, or a completion seen, resets
//! nothing.

use std::sync::atomic::{AtomicU32, Ordering::SeqCst};
use std::sync::Arc;

use nonos_libc::{entropy, said, Deadline};

use super::bus_master_tests::claimed;
use super::memory::{Memory, RX_RING_PHYS};
use super::model::Live;
use crate::constants::ctrl::STATUS_LU;
use crate::constants::regs::*;
use crate::init::finish;
use crate::server::tx_watch::check;
use crate::setup::Driver;

/// Up, three frames the part never completes, the deadline past.
pub fn stalled(link: u32) -> (Live, Arc<AtomicU32>, Driver, Memory) {
    let (bar, part, resets) = claimed(|_| {});
    let mut mem = Memory::new();
    let mut d = finish(mem.driver(&bar)).expect("brought up");
    bar.present32(REG_STATUS, link);
    for _ in 0..3 {
        d.tx.post(60);
    }
    d.tx_watch = Deadline::after_ms(0);
    (part, resets, d, mem)
}

#[test]
fn a_transmitter_stalled_with_the_link_up_is_reset_and_its_rings_restarted() {
    let _turn = entropy(true);
    let (_part, resets, mut d, _mem) = stalled(STATUS_LU);
    let done = resets.load(SeqCst);
    assert!(check(&mut d, false), "a stall past the bound is a hang");
    assert_eq!(resets.load(SeqCst), done + 1, "the part was reset");
    assert_eq!((d.tx.tail, d.tx.clean, d.rx.head), (0, 0, 0), "rings start over");
    // SAFETY: the window covers both queue blocks.
    let (rdbal, rxdctl, txdctl) =
        unsafe { (d.regs.r32(REG_RDBAL), d.regs.r32(REG_RXDCTL), d.regs.r32(REG_TXDCTL)) };
    assert_eq!(rdbal, RX_RING_PHYS as u32);
    assert_ne!(rxdctl & txdctl & (1 << 25), 0, "both queues back on");
    assert_eq!(said().last().unwrap(), "igc: tx hang, reset: done\n");
}

#[test]
fn a_full_ring_with_the_link_down_or_a_completion_seen_is_left_alone() {
    let _turn = entropy(true);
    let (_part, resets, mut d, _mem) = stalled(0);
    let done = resets.load(SeqCst);
    assert!(!check(&mut d, false), "the part sends once the link returns");
    assert!(!d.tx_watch.expired(), "the clock starts again from link down");
    d.tx_watch = Deadline::after_ms(0);
    assert!(!check(&mut d, true), "a completion is a sign of life");
    assert_eq!((resets.load(SeqCst), d.tx.tail), (done, 3), "nothing reset");
}
