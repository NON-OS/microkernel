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
//! The reset stage against the register model: the quiet, the master stop
//! that gates the reset, and the NVM auto-read that is waited for but not
//! required. The ordering is in reset_order_tests, the 82575 with no NVM in
//! reset_nvm_tests.

use std::time::Duration;

use nonos_libc::said;

use crate::bring::quiet::MASTER_STUCK;
use crate::bring::reset::run;
use crate::bring::reset::AUTO_READ_LATE;
use crate::facts::bounds::MASTER_DISABLE_MS;
use crate::facts::mac::*;
use crate::facts::parts::Part;
use crate::facts::regs::*;
use crate::facts::tx::TCTL_PSP;

use super::model::{regs, timed, window};

#[test]
fn a_part_that_stops_mastering_is_quiesced_and_reset() {
    let bar = window();
    bar.present32(REG_EECD, EECD_AUTO_RD | EECD_PRES);
    bar.present32(REG_RCTL, 0x0400_8002);
    let start = said().len();
    assert_eq!(run(&regs(&bar), Part::I82576), Ok(()));
    let ctrl = bar.wrote32(REG_CTRL);
    assert_ne!(ctrl & CTRL_GIO_MASTER_DISABLE, 0);
    assert_ne!(ctrl & CTRL_RST, 0);
    assert_eq!(bar.wrote32(REG_IMC), 0xFFFF_FFFF);
    assert_eq!(bar.wrote32(REG_RCTL), 0, "receiver stopped");
    assert_eq!(bar.wrote32(REG_TCTL), TCTL_PSP, "transmitter stopped, PSP kept");
    assert!(said()[start..].iter().all(|l| !l.contains("auto read")));
}

#[test]
fn a_master_that_never_stops_ends_the_attempt_before_any_reset() {
    let bar = window();
    bar.present32(REG_STATUS, STATUS_GIO_MASTER_ENABLE);
    let (r, took) = timed(|| run(&regs(&bar), Part::I350));
    assert_eq!(r, Err(MASTER_STUCK));
    assert!(took >= Duration::from_millis(MASTER_DISABLE_MS));
    assert_eq!(bar.wrote32(REG_CTRL) & CTRL_RST, 0, "no reset was issued");
}

#[test]
fn a_flashless_i210_is_logged_and_the_reset_goes_on() {
    let bar = window();
    assert_eq!(run(&regs(&bar), Part::I210), Ok(()));
    assert!(said().contains(&format!("igb: {AUTO_READ_LATE}\n")));
    assert_eq!(bar.wrote32(REG_IMC), 0xFFFF_FFFF, "causes masked after it");
}
