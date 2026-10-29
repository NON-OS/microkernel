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

//! The battery as the kernel reports it (MkBatteryStatus form 1, the bar's
//! own call): the ACPI capsule's word while it is fresh, else an errno.
//! Pure, proved in apps_proofs; never a number the kernel did not give.

const ENODEV: i64 = -19;
const PRESENT: i64 = 1 << 8;
const CHARGING: i64 = 1 << 9;
const DISCHARGING: i64 = 1 << 10;
const AC_ONLINE: i64 = 1 << 11;
const AC_KNOWN: i64 = 1 << 12;

/// The three rows: charge, state, AC; "" where the row has nothing to say.
pub fn rows(rc: i64, pct: &mut [u8; 4]) -> [&[u8]; 3] {
    if rc == ENODEV || (rc >= 0 && rc & PRESENT == 0) {
        return [b"No battery", b"", ac(rc)];
    }
    if rc < 0 {
        return [b"unavailable", b"", b"unknown"];
    }
    let charge: &[u8] = match (rc & 0xFF) as u8 {
        p @ 0..=100 => percent(p, pct),
        _ => b"unknown",
    };
    let state: &[u8] = match (rc & CHARGING != 0, rc & DISCHARGING != 0) {
        (true, false) => b"Charging",
        (false, true) => b"Discharging",
        (false, false) => b"Not charging",
        (true, true) => b"unknown",
    };
    [charge, state, ac(rc)]
}

fn ac(rc: i64) -> &'static [u8] {
    match (rc >= 0 && rc & AC_KNOWN != 0, rc & AC_ONLINE != 0) {
        (true, true) => b"Online",
        (true, false) => b"Offline",
        _ => b"unknown",
    }
}

/// "7%", "57%", "100%".
fn percent(p: u8, buf: &mut [u8; 4]) -> &[u8] {
    let digits = [p / 100, p / 10 % 10, p % 10];
    let first = digits.iter().position(|d| *d != 0).unwrap_or(2);
    let mut n = 0;
    for d in &digits[first..] {
        buf[n] = b'0' + d;
        n += 1;
    }
    buf[n] = b'%';
    &buf[..n + 1]
}
