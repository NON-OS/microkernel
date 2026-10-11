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
//! ixgbe_reset_hw against the register model: the quiesce, the master stop
//! that gates the reset, and the double CTRL.RST. Its waits are in
//! reset_wait_tests, the ordering in reset_order_tests.

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;

use nonos_libc::said;

use crate::constants::ctrl::*;
use crate::constants::regs::*;
use crate::constants::timeouts::MASTER_DISABLE_MS;
use crate::init::reset::master::MASTER_STUCK;
use crate::init::reset::run;

use super::model::{live, regs, timed, window};
use super::part_model::part;

#[test]
fn a_running_part_is_quiesced_and_reset_twice() {
    let bar = window();
    bar.present32(REG_RXCTRL, RXCTRL_RXEN | 0x10);
    bar.present32(REG_TXDCTL, 1 << 25);
    bar.present32(REG_AUTOC, 0xC0DE);
    let resets = Arc::new(AtomicU32::new(0));
    let _part = live(&bar, part(0xC0DE, resets.clone()));
    let start = said().len();
    assert_eq!(run(&regs(&bar)), Ok(0xC0DE), "AUTOC as found");
    assert_eq!(resets.load(Ordering::SeqCst), 2, "the double reset");
    assert_eq!(bar.wrote32(REG_RXCTRL), 0x10, "receiver off, the rest kept");
    assert_eq!(bar.wrote32(REG_TXDCTL), 0, "queue 0 stopped");
    assert_eq!(bar.wrote32(REG_EIMC), 0xFFFF_0000);
    assert_eq!((bar.wrote32(REG_EIMC_EX0), bar.wrote32(REG_EIMC_EX1)), (!0, !0));
    assert!(said()[start..].iter().all(|l| !l.contains("auto read")));
}

#[test]
fn a_master_that_never_stops_ends_the_attempt_before_any_reset() {
    let bar = window();
    bar.present32(REG_STATUS, STATUS_GIO);
    let (r, took) = timed(|| run(&regs(&bar)));
    assert_eq!(r, Err(MASTER_STUCK));
    assert!(took >= Duration::from_millis(MASTER_DISABLE_MS));
    assert_eq!(bar.wrote32(REG_CTRL) & CTRL_RST, 0, "no reset was issued");
}
