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

//! IndexField and BankField as ACPI 6.5 describes them (19.6.64 and
//! 19.6.7): CMOS-style registers behind an index/data pair, each byte
//! reached by writing its index and then touching the data register, and
//! two banks behind one window chosen by a bank-select register.

use super::index_bank_rig::rig;
use super::run::ok;

#[test]
fn index_field_bytes_through_the_pair() {
    let mut r = rig();
    assert_eq!(ok(&mut r, "\\RC0").0, 0xAA);
    let start = r.probe.log.len();
    assert_eq!(ok(&mut r, "\\RC1").0, 0x1234, "two bytes, each by its index");
    let pairs: Vec<_> = r.probe.log[start..].iter().map(|l| (l.1, l.3)).collect();
    assert_eq!(pairs, [(0x70, Some(0x11)), (0x71, None), (0x70, Some(0x12)), (0x71, None)]);
    assert_eq!(ok(&mut r, "\\WC1").0, 0x5566, "written through the pair and read back");
}

#[test]
fn bank_fields_select_their_bank_first() {
    let mut r = rig();
    assert_eq!(ok(&mut r, "\\RB1").0, 0x11);
    assert_eq!(ok(&mut r, "\\RB2").0, 0x22);
    assert_eq!(ok(&mut r, "\\RB1").0, 0x11, "the bank is selected again each time");
    let selects: Vec<_> = r.probe.log.iter().filter(|l| l.1 == 0x81).map(|l| l.3).collect();
    assert_eq!(selects, [Some(1), Some(2), Some(1)]);
}
