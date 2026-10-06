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

//! The 8125 init (Linux rtl_hw_init_8125) and the window an 8125 needs.

use nonos_devmodel::FakeBar;
use nonos_libc::logged;

use super::model::window;
use crate::chip::{detect, ChipError, MacVersion};
use crate::constants::regs::{REG_CMD, REG_TX_CONFIG};
use crate::hw::init_8125;
use crate::hw::regs::{REG_INTR_MITIGATE, REG_MCU};
use crate::regs::Regs;

#[test]
fn the_8125_init_leaves_oob_stops_both_engines_and_ends_with_its_three_ocp_words() {
    let bar = window();
    bar.present8(REG_CMD, 0x0C);
    bar.present8(REG_MCU, 0x80 | 0x30 | 0x02);
    bar.present16(REG_INTR_MITIGATE, 0x0103);
    init_8125(&Regs::new(bar.base()), MacVersion(63));
    assert_eq!(bar.wrote8(REG_CMD) & 0x0C, 0, "TE and RE off");
    assert_eq!(bar.wrote8(REG_MCU) & 0x80, 0, "NOW_IS_OOB cleared");
    assert_eq!(bar.wrote32(0xB0), 0x8000_0000 | (0xC01E << 15) | 0x5555, "0xC01E last");
    assert!(!logged().iter().any(|l| l.contains("not empty")), "{:?}", logged());
}

#[test]
fn an_8125_whose_bar_does_not_reach_0x4802_is_refused() {
    let bar = FakeBar::new(0x100);
    bar.present32(REG_TX_CONFIG, 0x641 << 20);
    let err = detect(&Regs::new(bar.base()), 0x100, true).unwrap_err();
    assert_eq!(err, ChipError::BarTooSmall(0x100));
    assert_eq!(err.as_str(), "rtl8169 8125 bar too small");
    let last = logged().last().cloned().unwrap_or_default();
    assert_eq!(last, "rtl8169: an 8125 needs its 64 KiB bar, mapped 0x100, not started\n");
    let chip = detect(&Regs::new(bar.base()), 0x10000, true).expect("RTL8125B");
    assert_eq!(chip.ver, MacVersion(63));
}
