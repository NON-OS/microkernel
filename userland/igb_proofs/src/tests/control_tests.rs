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

//! The engines and link stages against whatever a previous owner left in
//! the registers.

use crate::bring::{engines, link};
use crate::facts::mac::*;
use crate::facts::parts::Part;
use crate::facts::regs::*;

use super::model::{regs, window};

#[test]
fn tctl_gets_psp_rtlc_ct15_and_en_over_any_old_collision_threshold() {
    let bar = window();
    bar.present32(REG_TCTL, 0x0003_FFF0);
    bar.present32(REG_TXDCTL, 1 << 25);
    engines::enable(&regs(&bar));
    let tctl = bar.wrote32(REG_TCTL);
    assert_eq!(tctl & 0xFF0, 15 << 4, "CT is igb's 15");
    assert_eq!(tctl & 0x0100_000A, 0x0100_000A, "RTLC, PSP, EN");
    assert_eq!(tctl & 0x0003_F000, 0x0003_F000, "COLD left as found");
    assert_eq!(bar.wrote32(REG_TXDCTL), 0, "queue off before the enable");
}

#[test]
fn rctl_gets_en_bam_secrc_and_loses_loopback_bad_packets_and_long_frames() {
    let bar = window();
    bar.present32(REG_RCTL, 0x0003_30E4);
    engines::enable(&regs(&bar));
    let rctl = bar.wrote32(REG_RCTL);
    assert_eq!(rctl & 0x0400_8002, 0x0400_8002, "SECRC, BAM, EN");
    assert_eq!(rctl & (0x3 << 6 | 0x3 << 12 | 1 << 2 | 0x3 << 16 | 1 << 5), 0);
    assert_eq!(bar.wrote32(REG_RXDCTL), 0);
    assert_eq!(bar.wrote32(0x5008), 0, "igb makes no RFCTL errata write");
}

/// Only the force bits are cleared; FD and the speed select stay as found
/// and mean nothing once FRCSPD and FRCDPX are clear.
#[test]
fn the_link_is_set_up_not_forced_and_firmware_told_a_driver_is_loaded() {
    let bar = window();
    let left = CTRL_FRCSPD | CTRL_FRCDPX | 0x1 | (1 << 8);
    bar.present32(REG_CTRL, left);
    bar.present32(REG_PHPM, PHPM_GO_LINKD);
    link::run(&regs(&bar), Part::I82576);
    let ctrl = bar.wrote32(REG_CTRL);
    assert_eq!(ctrl & (CTRL_FRCSPD | CTRL_FRCDPX), 0);
    assert_eq!(ctrl & (0x1 | 0x300), 0x1 | (1 << 8), "FD and speed select kept");
    assert_ne!(ctrl & CTRL_SLU, 0);
    assert_ne!(bar.wrote32(REG_CTRL_EXT) & CTRL_EXT_DRV_LOAD, 0);
    assert_eq!(bar.wrote32(REG_PHPM), PHPM_GO_LINKD, "an 82576 has no PHPM to write");
}
