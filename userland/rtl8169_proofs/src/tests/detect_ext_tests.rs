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

//! The extended chip id: read from TX_CONFIG_V2 (0x60b0), and only when the
//! mapped window reaches it.

use super::detect_tests::{bar_with, last_line};
use crate::chip::{detect, ChipError, MacVersion};
use crate::regs::Regs;

#[test]
fn the_extended_id_is_read_from_tx_config_v2_only_inside_the_window() {
    let bar = bar_with(0x10000, 0x7C80_0000);
    let chip = detect(&Regs::new(bar.base()), 0x10000, true).expect("RTL9151AS");
    assert_eq!((chip.ver, chip.name, chip.extended), (MacVersion(64), "RTL9151AS", true));
    bar.present32(0x60b0, 0x0000_0001);
    let err = detect(&Regs::new(bar.base()), 0x10000, true).unwrap_err();
    assert_eq!(err, ChipError::Unknown { xid: 1, extended: true });
    assert_eq!(last_line(), "rtl8169: unknown chip ext xid 0x1, not started\n");
    let small = bar_with(0x100, 0x7C80_0000);
    let err = detect(&Regs::new(small.base()), 0x100, true).unwrap_err();
    assert_eq!(err, ChipError::ExtendedOutOfWindow);
}
