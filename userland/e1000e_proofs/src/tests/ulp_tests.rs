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

//! Leaving ULP: through ME when its firmware runs, by the driver when not,
//! not at all on the four LPT parts Linux exempts, and named when ME never
//! finishes.

use std::sync::atomic::Ordering;
use std::time::Instant;

use crate::constants::regs_pch::{REG_FWSM, REG_H2ME};
use crate::constants::status::*;
use crate::constants::Family;
use crate::init::ulp::disable;
use crate::model::live::live;
use crate::model::memory::Memory;
use crate::model::part::Behaviour;
use crate::model::window::{pch, phy_at};

const ME_IN_ULP: u32 = FWSM_RSPCIPHY | FWSM_FW_VALID | FWSM_ULP_CFG_DONE;

#[test]
fn with_me_running_it_is_asked_and_waited_for() {
    let bar = pch();
    bar.present32(REG_FWSM, ME_IN_ULP);
    bar.present32(REG_H2ME, H2ME_ULP);
    let how = Behaviour::default();
    how.me_leaves_ulp.store(true, Ordering::SeqCst);
    let _part = live(&bar, phy_at(2), how);
    let mut mem = Memory::new();
    let d = mem.driver(&bar, Family::PchCnp, 0x15BB);
    assert_eq!(disable(&d), Ok(()));
    assert_eq!(bar.wrote32(REG_H2ME) & (H2ME_ULP | H2ME_ENFORCE_SETTINGS), 0);
}

#[test]
fn an_me_that_never_finishes_is_named_after_2500_ms() {
    let bar = pch();
    bar.present32(REG_FWSM, ME_IN_ULP);
    let mut mem = Memory::new();
    let d = mem.driver(&bar, Family::PchSpt, 0x156F);
    let start = Instant::now();
    assert_eq!(disable(&d), Err("ME did not clear ULP_CFG_DONE in 2500 ms"));
    assert!(start.elapsed().as_millis() >= 2500);
}

#[test]
fn the_exempt_lpt_parts_and_the_82574_are_left_alone() {
    let bar = pch();
    bar.present32(REG_FWSM, ME_IN_ULP);
    let mut mem = Memory::new();
    for (family, id) in
        [(Family::PchLpt, 0x153A), (Family::PchLpt, 0x15A1), (Family::I82574, 0x10D3)]
    {
        assert_eq!(disable(&mem.driver(&bar, family, id)), Ok(()));
    }
    assert_eq!(bar.wrote32(REG_H2ME), 0, "ME was never asked");
}
