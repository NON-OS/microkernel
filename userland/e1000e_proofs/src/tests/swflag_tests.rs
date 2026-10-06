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

//! The software flag: granted when it reads back, and refused on the clock
//! when firmware holds it. A flag that firmware will not let stick cannot be
//! modelled over plain memory (the driver's own write always reads back), so
//! that refusal is covered by reading the code, not by a test.

use std::time::Instant;

use crate::constants::regs::REG_EXTCNF_CTRL;
use crate::constants::status::EXTCNF_CTRL_SWFLAG;
use crate::constants::Family;
use crate::model::window::{i82574, pch};
use crate::regs::Regs;
use crate::swflag::{acquire, release};

#[test]
fn a_free_flag_is_taken_and_given_back() {
    for (bar, family) in [(pch(), Family::PchSpt), (i82574(), Family::I82574)] {
        let regs = Regs::new(bar.base());
        assert_eq!(acquire(&regs, family), Ok(()));
        assert_ne!(bar.wrote32(REG_EXTCNF_CTRL) & EXTCNF_CTRL_SWFLAG, 0);
        release(&regs);
        assert_eq!(bar.wrote32(REG_EXTCNF_CTRL) & EXTCNF_CTRL_SWFLAG, 0);
    }
}

#[test]
fn a_flag_firmware_holds_is_waited_for_100_ms_then_refused() {
    let bar = pch();
    bar.present32(REG_EXTCNF_CTRL, EXTCNF_CTRL_SWFLAG);
    let start = Instant::now();
    let r = acquire(&Regs::new(bar.base()), Family::PchCnp);
    assert_eq!(r, Err("swflag held by firmware for 100 ms"));
    assert!(start.elapsed().as_millis() >= 100);
    assert_ne!(bar.wrote32(REG_EXTCNF_CTRL) & EXTCNF_CTRL_SWFLAG, 0, "firmware's, untouched");
}
