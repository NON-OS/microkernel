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

//! The wiped power-off's other rules (F-88): a battery is critical only
//! while discharging, by its flag or its charge at the low level; and the
//! fixed button is the FADT's unless its flags name a control-method one.

use crate::off_rule::{battery_critical, fixed_button};

#[test]
fn critical_batteries() {
    let levels = [None, Some(0u32), Some(50), Some(100)];
    for (d, flag) in [(false, false), (false, true), (true, false), (true, true)] {
        for left in levels {
            for low in levels {
                let under = matches!((left, low), (Some(r), Some(l)) if r <= l);
                assert_eq!(battery_critical(d, flag, left, low), d && (flag || under));
            }
        }
    }
}

#[test]
fn the_fixed_button_is_the_fadts_unless_flagged() {
    let mut fadt = [0u8; 116];
    assert!(fixed_button(&fadt));
    fadt[112] = 1 << 4;
    assert!(!fixed_button(&fadt), "PWR_BUTTON set: a control-method button");
    fadt[112] = !(1u8 << 4);
    assert!(fixed_button(&fadt), "other flags do not matter");
    assert!(!fixed_button(&fadt[..115]), "a table too short to say");
}
