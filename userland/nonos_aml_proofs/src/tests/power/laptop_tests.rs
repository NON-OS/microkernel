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

//! The laptop's tables read as capsule_acpi reads them, folded into the
//! word the kernel keeps: the charge (2565 of 4500 mAh, 57%), its
//! direction, AC and the lid.

use super::laptop_read::{lid, read};
use super::laptop_regs::*;
use super::state::Power;
use super::word::*;
use crate::power::minutes_left;

const AT_57: [(u64, u16); 5] =
    [(RATE, 1500), (LEFT, 2565), (FULL, 4500), (DESIGN, 5000), (VOLTS, 11800)];

fn with(state: u16, bits: u16) -> Vec<(u64, u16)> {
    [&AT_57[..], &[(STATE, state), (BITS, bits)]].concat()
}

#[test]
fn charging_on_ac_with_the_lid_open() {
    let mut r = laptop(&with(2, 0b11));
    let p = read(&mut r);
    let b = &p.batteries[0];
    let (i, s) = (b.info.as_ref().unwrap(), b.status.unwrap());
    assert!(b.present && i.milliamp && s.charging && !s.discharging);
    assert_eq!((i.last_full, i.design_capacity, s.remaining), (Some(4500), Some(5000), Some(2565)));
    assert_eq!(i.model, b"BAT-57");
    assert_eq!(word(&p), 57 | BATTERY_PRESENT | CHARGING | AC_KNOWN | AC_ONLINE);
    assert_eq!(lid(&mut r), Some(true));
    assert_eq!(minutes_left(i, &s), Some(77), "to full: the word has no room for it");
}

#[test]
fn discharging_on_battery_and_the_lid_closed() {
    let mut r = laptop(&with(1, 0b00));
    let p = read(&mut r);
    assert_eq!(word(&p), 57 | BATTERY_PRESENT | DISCHARGING | AC_KNOWN);
    let bat = crate::consts::at(&r.aml, "\\_SB.LPCB.EC0.BAT0");
    assert_eq!((Some(p.batteries[0].id), p.batteries[0].told), (bat, 0), "the device, read once");
    assert!(p.reported.is_none() && p.read_ms == 0 && !p.low_told && !p.critical, "nothing sent");
    assert_eq!(lid(&mut r), Some(false));
}

#[test]
fn a_full_battery_on_ac_is_neither_charging_nor_discharging() {
    let full = [(STATE, 0), (BITS, 0b11), (LEFT, 4500), (FULL, 4500), (DESIGN, 5000)];
    assert_eq!(word(&read(&mut laptop(&full))), 100 | BATTERY_PRESENT | AC_KNOWN | AC_ONLINE);
}

#[test]
fn a_battery_that_reports_no_capacity_has_no_charge() {
    let empty = [(STATE, 1), (BITS, 0b10)];
    let w = word(&read(&mut laptop(&empty)));
    assert_eq!(w, UNKNOWN_PERCENT | BATTERY_PRESENT | DISCHARGING | AC_KNOWN, "never 0%");
}

#[test]
fn a_machine_with_no_battery_or_adapter_reports_none() {
    assert_eq!(word(&Power::default()), UNKNOWN_PERCENT, "not present, AC unknown");
}
