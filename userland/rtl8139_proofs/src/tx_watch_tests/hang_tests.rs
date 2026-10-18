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
//! part and starts over; link down, or a frame finished, resets nothing.

use nonos_libc::{entropy, logged, Deadline};

use crate::constants::regs::*;
use crate::init::finish;
use crate::part::{driver_over, part, part_with, RX_DA};
use crate::server::tx_watch::check;
use crate::setup::Driver;

/// Brought up, three frames the part never finishes, the deadline past.
pub fn stalled() -> Driver {
    let mut d = finish(driver_over(0)).expect("brought up");
    (d.tx_cur, d.rx_offset) = (3, 512);
    d.tx_watch = Deadline::after_ms(0);
    d
}

#[test]
fn a_transmitter_stalled_with_the_link_up_is_reset_and_started_over() {
    let _turn = entropy(true);
    let (_port, resets) = part_with(false, None);
    let mut d = stalled();
    let done = resets.get();
    assert!(check(&mut d, false), "a stall past the bound is a hang");
    assert_eq!(resets.get(), done + 1, "the part was reset");
    assert_eq!((d.tx_cur, d.tx_dirty, d.rx_offset), (0, 0, 0), "started over");
    assert_eq!(d.pio.r32(REG_RBSTART), Ok(RX_DA as u32));
    let on = CMD_RX_ENABLE | CMD_TX_ENABLE;
    assert_eq!(d.pio.r8(REG_CMD).map(|c| c & on), Ok(on), "both engines back on");
    assert_eq!(logged().last().unwrap(), "driver.rtl8139: tx hang, reset: done\n");
}

#[test]
fn slots_held_with_the_link_down_or_a_frame_finished_are_left_alone() {
    let _turn = entropy(true);
    let (_port, resets) = part_with(false, None);
    let (mut d, done) = (stalled(), resets.get());
    d.pio.w8(REG_MSR, MSR_LINK_BAD).expect("link down");
    assert!(!check(&mut d, false), "the part sends once the link returns");
    assert!(!d.tx_watch.expired(), "the clock starts again from link down");
    d.pio.w8(REG_MSR, 0).expect("link up");
    d.tx_watch = Deadline::after_ms(0);
    assert!(!check(&mut d, true), "a finished frame is a sign of life");
    assert_eq!((resets.get(), d.tx_cur), (done, 3), "nothing reset or dropped");
}

#[test]
fn a_reset_that_fails_is_named_and_tried_again_later() {
    let _turn = entropy(true);
    let _port = part(false);
    let mut d = stalled();
    let _stuck = part(true);
    assert!(check(&mut d, false), "the hang is acted on");
    let line = "driver.rtl8139: tx hang, reset: rtl8139 reset timeout\n";
    assert_eq!(logged().last().unwrap(), line);
    assert!(!d.tx_watch.expired(), "the next try waits a whole bound");
}
