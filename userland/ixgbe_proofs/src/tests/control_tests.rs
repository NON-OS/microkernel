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
//! The MAC settings, the 82599's link mode across the reset, and the
//! hand-over from firmware, against whatever a previous owner left.

use crate::bring::settings;
use crate::facts::mac::*;
use crate::facts::parts::Part;
use crate::facts::regs::*;

use super::model::{regs, window};

#[test]
fn hlreg0_strips_and_adds_crc_pads_and_refuses_jumbo_frames() {
    let bar = window();
    bar.present32(REG_HLREG0, HLREG0_JUMBOEN | 0x0000_2000);
    settings::program(&regs(&bar));
    let hl = bar.wrote32(REG_HLREG0);
    assert_eq!(hl & HLREG0_JUMBOEN, 0);
    let on = HLREG0_TXCRCEN | HLREG0_RXCRCSTRP | HLREG0_TXPADEN;
    assert_eq!(hl & on, on);
    assert_eq!(hl & 0x2000, 0x2000, "other bits as found");
}

#[test]
fn rdrxctl_strips_crc_too_and_carries_the_bits_hardware_requires() {
    let bar = window();
    bar.present32(REG_RDRXCTL, RDRXCTL_RSCFRSTSIZE | 1);
    settings::program(&regs(&bar));
    let r = bar.wrote32(REG_RDRXCTL);
    assert_eq!(r & RDRXCTL_RSCFRSTSIZE, 0);
    assert_eq!(r & 1, 1, "other bits as found");
    let on = RDRXCTL_CRCSTRIP | RDRXCTL_RSCACKC | RDRXCTL_FCOE_WRFIX;
    assert_eq!(r & on, on);
    assert_ne!(bar.wrote32(REG_RSCDBU) & RSCDBU_RSCACKDIS, 0);
    assert_ne!(bar.wrote32(REG_FCTRL) & FCTRL_BAM, 0, "broadcast taken");
}

#[test]
fn the_82599s_autoc_is_put_back_and_autonegotiation_restarted() {
    let bar = window();
    bar.present32(REG_AUTOC, 0x0000_A000);
    crate::bring::mode::restore(&regs(&bar), Part::I82599, 0x0000_C000);
    assert_eq!(bar.wrote32(REG_AUTOC), 0x0000_C000 | AUTOC_AN_RESTART);
    // The same mode is left alone: no restart drops a link that is up.
    let same = window();
    same.present32(REG_AUTOC, 0x0000_C000);
    crate::bring::mode::restore(&regs(&same), Part::I82599, 0x0000_C000);
    assert_eq!(same.wrote32(REG_AUTOC), 0x0000_C000);
    // The X540 and X550 run their own copper PHY.
    for part in [Part::X540, Part::X550] {
        let copper = window();
        copper.present32(REG_AUTOC, 0x0000_A000);
        crate::bring::mode::restore(&regs(&copper), part, 0x0000_C000);
        assert_eq!(copper.wrote32(REG_AUTOC), 0x0000_A000);
    }
}
