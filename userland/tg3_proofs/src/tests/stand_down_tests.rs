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

//! tg3_halt for a shutdown on a part left running: every engine and the
//! receiver stopped before the chip reset that follows, so no DMA reaches
//! a grant after the broker takes it back.

use nonos_libc::said;

use crate::constants::blocks::*;
use crate::constants::grc::{GRC_MISC_CFG, GRC_MISC_CFG_CORECLK_RESET};
use crate::constants::mac::*;
use crate::init::stand_down;

use super::build::{LOM, REV_57766};
use super::config_replay::seed;
use super::memory::Memory;
use super::model::window;

#[test]
fn a_running_part_is_stopped_then_reset() {
    let bar = window();
    let mut m = Memory::new();
    let mut d = m.driver(&bar, REV_57766, LOM);
    seed();
    let on = [RDMAC_MODE, WDMAC_MODE, SNDDATAI_MODE, RCVDBDI_MODE, MAC_TX_MODE, MAC_RX_MODE];
    // SAFETY: every offset is inside the window.
    unsafe {
        for at in on {
            d.regs.w32(at, MODE_ENABLE);
        }
    }
    d.modes.rx_mode = RX_MODE_ENABLE;
    d.modes.tx_mode = TX_MODE_ENABLE;
    stand_down::run(&mut d);
    for at in on {
        assert_eq!(bar.wrote32(at) & MODE_ENABLE, 0, "{at:#x} still on");
    }
    let reset = bar.wrote32(GRC_MISC_CFG) & GRC_MISC_CFG_CORECLK_RESET;
    assert_eq!(reset, GRC_MISC_CFG_CORECLK_RESET);
    assert!(said().iter().all(|l| !l.contains("stand-down reset")));
}
