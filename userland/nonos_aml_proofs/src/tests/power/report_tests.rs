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

//! The word the capsule folds, through the kernel's report (believed three
//! minutes) to MkBatteryStatus's state form, and what the bar and About
//! say of it. One test: the report is one static the cases would share.

use super::about_battery::rows;
use super::battery_text::{label, Battery, LABEL_MAX};
use super::laptop_read::read;
use super::laptop_regs::*;
use super::power_report::{self as k, battery, record, state};
use super::word::{self as c, word};

fn bar(rc: i64) -> String {
    let mut buf = [0u8; LABEL_MAX];
    let n = label(Battery::from_state(rc), &mut buf);
    String::from_utf8(buf[..n].to_vec()).unwrap()
}

fn about(rc: i64) -> [String; 3] {
    let mut pct = [0u8; 4];
    rows(rc, &mut pct).map(|r| String::from_utf8(r.to_vec()).unwrap())
}

#[test]
fn the_laptop_reads_57_percent_charging_on_the_bar_and_in_about() {
    let bits = [c::BATTERY_PRESENT, c::CHARGING, c::DISCHARGING, c::AC_ONLINE, c::AC_KNOWN];
    let kernel = [k::BATTERY_PRESENT, k::CHARGING, k::DISCHARGING, k::AC_ONLINE, k::AC_KNOWN];
    assert_eq!(bits, kernel, "the capsule's bits are the kernel's");
    let regs = [(STATE, 2), (BITS, 0b11), (RATE, 1500), (LEFT, 2565), (FULL, 4500)];
    let w = word(&read(&mut laptop(&regs)));
    assert_eq!(w & !k::KNOWN_BITS, 0, "MkAcpiPower takes it");
    assert_eq!(state(1_000), None, "nothing believed before a report");
    record(w, 10_000);
    assert_eq!(battery(10_000), k::Battery::Percent(57));
    assert_eq!(state(190_000), Some(w), "three minutes");
    assert_eq!(bar(w as i64), "57% Charging");
    assert_eq!(Battery::from_state(w as i64).percent(), Some(57), "the gauge's fill");
    assert_eq!(about(w as i64), ["57%", "Charging", "Online"]);
    assert_eq!(state(190_001), None, "stale: the kernel falls back to its scan");
    let unplugged = w & !(k::CHARGING | k::AC_ONLINE) | k::DISCHARGING;
    record(unplugged, 200_000);
    assert_eq!(bar(state(200_000).unwrap() as i64), "57%");
    assert_eq!(about(unplugged as i64), ["57%", "Discharging", "Offline"]);
    // QEMU q35: a DSDT with no PNP0C0A, so the capsule reports none.
    record(word(&Default::default()), 300_000);
    assert_eq!(battery(300_000), k::Battery::Absent);
    assert_eq!(bar(state(300_000).unwrap() as i64), "No battery");
    assert_eq!(about(state(300_000).unwrap() as i64), ["No battery", "", "unknown"]);
    // Errnos, as MkBatteryStatus answers without a fresh report.
    assert_eq!((bar(-19), bar(-95)), ("No battery".into(), "Battery status unavailable".into()));
    assert_eq!(about(-95), ["unavailable", "", "unknown"]);
    let unknown = k::BATTERY_PRESENT | k::PERCENT_MASK;
    assert_eq!(
        (bar(unknown as i64), &about(unknown as i64)[0][..]),
        ("Battery status unavailable".into(), "unknown")
    );
}
