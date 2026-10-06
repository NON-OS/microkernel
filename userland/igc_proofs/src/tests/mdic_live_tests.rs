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

//! MDIC cycles against a modelled PHY, and the two ways a cycle fails: the
//! part never sets READY, or sets it with ERROR.

use std::sync::atomic::Ordering;
use std::time::Duration;

use crate::constants::phy::{MDIC_ERROR, MDIC_READY, PHY_CONTROL};
use crate::constants::regs::REG_MDIC;
use crate::constants::timeouts::MDIC_MS;
use crate::init::phy::mdic::{read, write, READ_FAILED, READ_SLOW, WRITE_SLOW};

use super::model::{live, regs, stable32, timed, window};
use super::phy_model::{bmcr, phy, BMCR_POWERED_DOWN};

#[test]
fn a_read_returns_what_the_phy_holds_and_a_write_stores_it() {
    let bar = window();
    let held = bmcr(BMCR_POWERED_DOWN);
    let _part = live(&bar, phy(held.clone()));
    assert_eq!(read(&regs(&bar), PHY_CONTROL), Ok(BMCR_POWERED_DOWN as u16));
    assert_eq!(write(&regs(&bar), PHY_CONTROL, 0x1340), Ok(()));
    assert_eq!(held.load(Ordering::SeqCst), 0x1340);
}

#[test]
fn a_phy_that_never_answers_times_out_on_the_clock() {
    let bar = window();
    let (r, took) = timed(|| read(&regs(&bar), PHY_CONTROL));
    assert_eq!(r, Err(READ_SLOW));
    assert!(took >= Duration::from_millis(MDIC_MS), "gave up after {took:?}");
    let (w, _) = timed(|| write(&regs(&bar), PHY_CONTROL, 0));
    assert_eq!(w, Err(WRITE_SLOW));
}

#[test]
fn a_cycle_ending_in_error_is_named() {
    let bar = window();
    let _part = live(&bar, |b| {
        let Some(cmd) = stable32(b, REG_MDIC) else { return };
        if cmd != 0 && cmd & MDIC_READY == 0 {
            b.present32(REG_MDIC, cmd | MDIC_READY | MDIC_ERROR);
        }
    });
    assert_eq!(read(&regs(&bar), PHY_CONTROL), Err(READ_FAILED));
}
