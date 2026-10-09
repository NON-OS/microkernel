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

//! MDIC words against igc_defines.h and the BMCR edit the power-up makes.

use crate::constants::phy::*;
use crate::init::phy::mdic_word::{outcome, read_cmd, write_cmd, Mdic, REG_RANGE};
use crate::init::phy::power_up::bmcr_on;

#[test]
fn fields_are_the_igc_defines_h_values() {
    assert_eq!(MDIC_DATA_MASK, 0x0000_FFFF);
    assert_eq!(0x1F << MDIC_REG_SHIFT, 0x001F_0000);
    assert_eq!(0x1F << MDIC_PHY_SHIFT, 0x03E0_0000);
    assert_eq!(MDIC_OP_WRITE, 0x0400_0000);
    assert_eq!(MDIC_OP_READ, 0x0800_0000);
    assert_eq!(MDIC_READY, 0x1000_0000);
    assert_eq!(MDIC_ERROR, 0x4000_0000);
    assert_eq!(PHY_ADDR, 0, "igc leaves hw->phy.addr at zero");
}

#[test]
fn command_vectors() {
    assert_eq!(read_cmd(PHY_CONTROL), Ok(0x0800_0000));
    assert_eq!(read_cmd(0x02), Ok(0x0802_0000), "PHY_ID1");
    assert_eq!(write_cmd(PHY_CONTROL, 0x1340), Ok(0x0400_1340));
    assert_eq!(write_cmd(0x1F, 0xFFFF), Ok(0x041F_FFFF));
}

#[test]
fn a_register_past_0x1f_is_refused_not_masked() {
    for reg in [0x20u32, 0x21, 0xFFFF, u32::MAX] {
        assert_eq!(read_cmd(reg), Err(REG_RANGE));
        assert_eq!(write_cmd(reg, 0), Err(REG_RANGE));
    }
}

#[test]
fn outcome_vectors() {
    assert_eq!(outcome(0x0800_0000), Mdic::Busy);
    assert_eq!(outcome(0x4800_1234), Mdic::Busy, "ERROR counts only with READY");
    assert_eq!(outcome(0x1800_1234), Mdic::Done(0x1234));
    assert_eq!(outcome(0x5800_1234), Mdic::Failed);
}

#[test]
fn power_up_clears_power_down_and_restarts_autoneg_keeping_the_rest() {
    assert_eq!(MII_CR_POWER_DOWN, 0x0800);
    assert_eq!(MII_CR_AUTO_NEG_EN, 0x1000);
    assert_eq!(MII_CR_RESTART_AUTO_NEG, 0x0200);
    assert_eq!(bmcr_on(0x1940), 0x1340);
    assert_eq!(bmcr_on(0x0800), 0x1200);
    for bmcr in 0..=u16::MAX {
        let on = bmcr_on(bmcr);
        assert_eq!(on & 0x0800, 0);
        assert_eq!(on & 0x1200, 0x1200);
        assert_eq!(on & !0x1A00, bmcr & !0x1A00, "other bits as the PHY had them");
    }
}
