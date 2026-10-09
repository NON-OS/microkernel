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

//! Leaving ULP with no ME running: the driver does it itself.

use crate::constants::ctrl::CTRL_EXT_FORCE_SMBUS;
use crate::constants::pch_bits::FEXTNVM7_DISABLE_SMB_PERST;
use crate::constants::phy::{CV_SMB_CTRL, HV_PM_CTRL, I218_ULP_CONFIG1};
use crate::constants::regs::REG_CTRL_EXT;
use crate::constants::regs_pch::REG_FEXTNVM7;
use crate::constants::Family;
use crate::init::ulp::disable;
use crate::model::live::live;
use crate::model::memory::Memory;
use crate::model::part::Behaviour;
use crate::model::window::{pch, phy_at};

#[test]
fn without_me_the_driver_unforces_smbus_restores_k1_and_restarts_ulp_config() {
    let bar = pch();
    bar.present32(REG_CTRL_EXT, bar.wrote32(REG_CTRL_EXT) | CTRL_EXT_FORCE_SMBUS);
    bar.present32(REG_FEXTNVM7, FEXTNVM7_DISABLE_SMB_PERST);
    let part = live(&bar, phy_at(2), Behaviour::default());
    part.phy.set(1, CV_SMB_CTRL.0, CV_SMB_CTRL.1, 0x0001);
    part.phy.set(1, I218_ULP_CONFIG1.0, I218_ULP_CONFIG1.1, 0x1D74);
    let mut mem = Memory::new();
    let d = mem.driver(&bar, Family::PchTgp, 0x15FB);
    assert_eq!(disable(&d), Ok(()));
    let at = |r: (u16, u32)| part.phy.get(1, r.0, r.1);
    assert_eq!(at(CV_SMB_CTRL), 0, "PHY out of forced SMBus");
    assert_eq!(at(HV_PM_CTRL), 0x4000, "K1 back on");
    assert_eq!(at(I218_ULP_CONFIG1), 0x0001, "ULP bits cleared, START set");
    assert_eq!(bar.wrote32(REG_CTRL_EXT) & CTRL_EXT_FORCE_SMBUS, 0);
    assert_eq!(bar.wrote32(REG_FEXTNVM7) & FEXTNVM7_DISABLE_SMB_PERST, 0);
}
