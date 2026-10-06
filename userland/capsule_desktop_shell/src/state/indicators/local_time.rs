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

use nonos_libc::{mk_time_rtc, RtcTime};

// The wall clock shifted by the user's whole-hour offset, with the date
// carried across midnight and month ends. None when the RTC does not answer.
pub fn now(offset_hours: i8) -> Option<RtcTime> {
    let mut t = RtcTime::default();
    if mk_time_rtc(&mut t as *mut RtcTime) != 0 {
        return None;
    }
    let mut hour = t.hour as i32 + offset_hours as i32;
    let mut days = days_from_civil(t.year as i32, t.month as i32, t.day as i32);
    days += hour.div_euclid(24) as i64;
    hour = hour.rem_euclid(24);
    let (y, m, d) = civil_from_days(days);
    Some(RtcTime { year: y as u16, month: m as u8, day: d as u8, hour: hour as u8, ..t })
}

// Days since 1970-01-01 in the proleptic Gregorian calendar (Hinnant).
fn days_from_civil(y: i32, m: i32, d: i32) -> i64 {
    let y = if m <= 2 { y - 1 } else { y } as i64;
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let mp = (m as i64 + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d as i64 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (if m <= 2 { yoe + era * 400 + 1 } else { yoe + era * 400 }, m, d)
}
