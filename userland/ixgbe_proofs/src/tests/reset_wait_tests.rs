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
//! The reset's own waits: a CTRL.RST that never clears ends the attempt, an
//! EEPROM auto-read that never finishes is logged and gone past.

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;

use nonos_libc::said;

use crate::constants::ctrl::*;
use crate::constants::regs::*;
use crate::constants::timeouts::RESET_CLEAR_MS;
use crate::init::reset::global::{AUTO_READ_LATE, RESET_STUCK};
use crate::init::reset::run;

use super::model::{live, regs, timed, window};

#[test]
fn a_reset_that_never_clears_ends_the_attempt() {
    let bar = window();
    let (r, took) = timed(|| run(&regs(&bar)));
    assert_eq!(r, Err(RESET_STUCK));
    assert!(took >= Duration::from_millis(RESET_CLEAR_MS));
}

#[test]
fn a_late_auto_read_is_logged_and_the_reset_goes_on() {
    let bar = window();
    let resets = Arc::new(AtomicU32::new(0));
    let _part = live(&bar, move |b| {
        let ctrl = b.wrote32(REG_CTRL);
        if ctrl & CTRL_RST != 0 {
            b.present32(REG_CTRL, ctrl & !CTRL_RST);
            resets.fetch_add(1, Ordering::SeqCst);
        }
    });
    assert!(run(&regs(&bar)).is_ok());
    assert!(said().contains(&format!("ixgbe: {AUTO_READ_LATE}\n")));
}
