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

//! Resets that stop: one that never clears, and an 82574 NVM read that
//! never ends, each named; the flag let go either way.

use crate::constants::ctrl::CTRL_PHY_RST;
use crate::constants::regs::REG_EXTCNF_CTRL;
use crate::constants::status::EXTCNF_CTRL_SWFLAG;
use crate::constants::Family;
use crate::init::reset::run;
use crate::model::live::live;
use crate::model::part::Behaviour;
use crate::model::window::{i82574, pch, phy_at};
use crate::regs::Regs;
use std::sync::atomic::Ordering;

#[test]
fn a_reset_that_never_clears_is_named_and_the_flag_let_go() {
    let bar = pch();
    let how = Behaviour::default();
    how.reset_stuck.store(true, Ordering::SeqCst);
    let _part = live(&bar, phy_at(2), how);
    assert_eq!(run(&Regs::new(bar.base()), Family::PchSpt), Err("reset did not clear in 50 ms"));
    assert_eq!(bar.wrote32(REG_EXTCNF_CTRL) & EXTCNF_CTRL_SWFLAG, 0);
}

#[test]
fn the_82574_waits_for_its_nvm_read_and_names_one_that_never_ends() {
    let bar = i82574();
    let part = live(&bar, phy_at(1), Behaviour::default());
    assert_eq!(run(&Regs::new(bar.base()), Family::I82574), Ok(()));
    assert_eq!(part.how.resets.lock().unwrap()[0] & CTRL_PHY_RST, 0, "no PHY reset here");
    part.how.no_auto_rd.store(true, Ordering::SeqCst);
    assert_eq!(run(&Regs::new(bar.base()), Family::I82574), Err("EECD.AUTO_RD not set in 20 ms"));
}
