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

//! igc_get_hw_semaphore_i225 against SWSM: taken and put back, and a stale
//! SMBI waited out for the clock bound and then cleared once, as Linux does.
//!
//! Not proven here: an agent that never lets go of the bit. The model shares
//! memory with the driver, so the driver's own write reads back before any
//! concurrent model can undo it; that refusal rests on `until`, whose
//! timeout is proven in wait_tests.

use std::time::Duration;

use crate::constants::phy::{SWSM_SMBI, SWSM_SWESMBI};
use crate::constants::regs::REG_SWSM;
use crate::constants::timeouts::SEMAPHORE_MS;
use crate::init::phy::semaphore::{get, put};

use super::model::{regs, timed, window};

#[test]
fn a_free_semaphore_is_taken_and_put_back() {
    let bar = window();
    assert_eq!(get(&regs(&bar)), Ok(()));
    assert_ne!(bar.wrote32(REG_SWSM) & SWSM_SWESMBI, 0, "SWESMBI latched");
    put(&regs(&bar));
    assert_eq!(bar.wrote32(REG_SWSM) & (SWSM_SMBI | SWSM_SWESMBI), 0);
}

#[test]
fn a_stale_smbi_is_cleared_once_as_linux_does() {
    let bar = window();
    bar.present32(REG_SWSM, SWSM_SMBI);
    let (r, took) = timed(|| get(&regs(&bar)));
    assert_eq!(r, Ok(()));
    assert!(took >= Duration::from_millis(SEMAPHORE_MS), "waited it out first");
}
