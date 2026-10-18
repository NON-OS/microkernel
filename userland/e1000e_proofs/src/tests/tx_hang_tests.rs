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

use nonos_libc::{entropy, logged, Deadline};

use super::bus_master_tests::{claimed, up};
use super::probe::resets;
use crate::constants::regs::{REG_RCTL, REG_RDBAL, REG_STATUS, REG_TCTL};
use crate::constants::rxtx::{RCTL_EN, TCTL_EN};
use crate::constants::status::STATUS_LU;
use crate::model::live::Live;
use crate::model::memory::{Memory, RX_RING_PHYS};
use crate::server::tx_watch::check;
use crate::setup::Driver;

/// Up, three frames the part never completes, the deadline past.
pub fn stalled(link: u32) -> (Live, Driver, Memory) {
    let (bar, part) = claimed();
    let mut mem = Memory::new();
    let mut d = up(&bar, &mut mem).expect("brought up");
    bar.present32(REG_STATUS, link);
    for _ in 0..3 {
        d.tx.post(60);
    }
    d.tx_watch = Deadline::after_ms(0);
    (part, d, mem)
}

#[test]
fn a_transmitter_stalled_with_the_link_up_is_reset_and_its_rings_restarted() {
    let _turn = entropy(true);
    let (part, mut d, _mem) = stalled(STATUS_LU);
    let done = resets(&part.how);
    assert!(check(&mut d, false), "a stall past the bound is a hang");
    assert_eq!(resets(&part.how), done + 1, "the part was reset");
    assert_eq!((d.tx.tail, d.tx.clean, d.rx.head), (0, 0, 0), "rings start over");
    // SAFETY: RDBAL, RCTL and TCTL lie inside the window.
    let (rdbal, rctl, tctl) =
        unsafe { (d.regs.r32(REG_RDBAL), d.regs.r32(REG_RCTL), d.regs.r32(REG_TCTL)) };
    assert_eq!(rdbal, RX_RING_PHYS as u32);
    assert_eq!((rctl & RCTL_EN, tctl & TCTL_EN), (RCTL_EN, TCTL_EN), "both units on");
    assert_eq!(logged().last().unwrap(), "e1000e: tx hang, reset: done");
}

#[test]
fn a_full_ring_with_the_link_down_or_a_completion_seen_is_left_alone() {
    let _turn = entropy(true);
    let (part, mut d, _mem) = stalled(0);
    let done = resets(&part.how);
    assert!(!check(&mut d, false), "the part sends once the link returns");
    assert!(!d.tx_watch.expired(), "the clock starts again from link down");
    d.tx_watch = Deadline::after_ms(0);
    assert!(!check(&mut d, true), "a completion is a sign of life");
    assert_eq!((resets(&part.how), d.tx.tail), (done, 3), "nothing reset");
}
