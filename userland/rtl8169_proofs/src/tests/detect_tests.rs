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

//! Detection against a window: the line it logs, and the refusals.

use nonos_devmodel::FakeBar;
use nonos_libc::logged;

use crate::chip::{detect, ChipError, MacVersion};
use crate::constants::regs::REG_TX_CONFIG;
use crate::regs::Regs;

pub(super) fn bar_with(len: usize, txconfig: u32) -> FakeBar {
    let bar = FakeBar::new(len);
    bar.present32(REG_TX_CONFIG, txconfig);
    bar
}

pub(super) fn last_line() -> String {
    logged().last().cloned().unwrap_or_default()
}

#[test]
fn a_known_chip_is_logged_with_its_version_xid_and_name() {
    let bar = bar_with(0x100, 0x5410_0700);
    let chip = detect(&Regs::new(bar.base()), 0x100, true).expect("8168h");
    assert_eq!(chip.ver, MacVersion(46));
    assert_eq!(last_line(), "rtl8169: chip RTL_GIGA_MAC_VER_46 xid 0x541 (RTL8168h/8111h)\n");
    let bar = bar_with(0x100, 0x0080_0000);
    detect(&Regs::new(bar.base()), 0x100, true).expect("8169s");
    assert_eq!(last_line(), "rtl8169: chip RTL_GIGA_MAC_VER_02 xid 0x8 (RTL8169s)\n");
}

#[test]
fn an_unknown_xid_is_refused_by_name_and_logged() {
    let bar = bar_with(0x100, 0x7C00_0000);
    let err = detect(&Regs::new(bar.base()), 0x100, true).unwrap_err();
    assert_eq!(err, ChipError::Unknown { xid: 0x7c0, extended: false });
    assert_eq!(err.as_str(), "rtl8169 unknown chip xid");
    assert_eq!(last_line(), "rtl8169: unknown chip xid 0x7c0, not started\n");
}

#[test]
fn a_bar_that_reads_all_ones_and_the_5g_and_10g_parts_are_refused() {
    let bar = bar_with(0x100, 0xFFFF_FFFF);
    assert_eq!(detect(&Regs::new(bar.base()), 0x100, true), Err(ChipError::ReadFailed));
    for xid in [0x64a_u32, 0x6c9] {
        let bar = bar_with(0x100, xid << 20);
        let err = detect(&Regs::new(bar.base()), 0x100, true).unwrap_err();
        assert!(matches!(err, ChipError::Unsupported(_)), "xid {xid:#x}");
        assert!(last_line().ends_with("not supported by this driver, not started\n"));
    }
}
