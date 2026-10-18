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

//! The bus speed a declared device asks for, and the SDA hold register a
//! core older than 1.11a does not have.

use crate::constants::{IC_COMP_VERSION, IC_CON, IC_CON_SPEED_FAST, IC_CON_SPEED_STD, IC_SDA_HOLD};
use crate::init::{bring_up, BusSetup};
use crate::model::live;
use crate::setup::{regs, SETUP};

const SPEED_MASK: u32 = 3 << 1;

#[test]
fn a_device_that_declares_100_khz_gets_standard_mode() {
    let bar = live();
    let setup = BusSetup { standard_mode: true, ..SETUP };
    bring_up(regs(&bar), setup).expect("bring-up");
    assert_eq!(bar.wrote32(IC_CON as usize) & SPEED_MASK, IC_CON_SPEED_STD);
    let bar = live();
    bring_up(regs(&bar), SETUP).expect("bring-up");
    assert_eq!(bar.wrote32(IC_CON as usize) & SPEED_MASK, IC_CON_SPEED_FAST);
}

#[test]
fn a_core_older_than_1_11a_has_no_sda_hold_register_and_it_is_left_alone() {
    let bar = live();
    bar.present32(IC_COMP_VERSION as usize, 0x3130_302A);
    bring_up(regs(&bar), SETUP).expect("bring-up");
    assert_eq!(bar.wrote32(IC_SDA_HOLD as usize), 0);
}
