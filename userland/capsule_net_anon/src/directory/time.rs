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

//! Turning a document timestamp into seconds since the epoch.

use super::number::decimal;

/// Parse `YYYY-MM-DD HH:MM:SS`, which every directory document uses and always
/// in UTC. Returns seconds since the epoch, or `None` if any field is missing
pub fn parse(text: &[u8]) -> Option<u64> {
    if text.len() < 19 || text[4] != b'-' || text[7] != b'-' {
        return None;
    }
    if text[10] != b' ' || text[13] != b':' || text[16] != b':' {
        return None;
    }
    let year = decimal(&text[0..4])?;
    let month = decimal(&text[5..7])?;
    let day = decimal(&text[8..10])?;
    let hour = decimal(&text[11..13])?;
    let minute = decimal(&text[14..16])?;
    let second = decimal(&text[17..19])?;
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    if hour > 23 || minute > 59 || second > 60 {
        return None;
    }
    let days = days_from_civil(year as i64, month as i64, day as i64)?;
    let seconds = days.checked_mul(86_400)?;
    u64::try_from(seconds + (hour * 3600 + minute * 60 + second) as i64).ok()
}

/*
 * Howard Hinnant's days_from_civil, which shifts the year so that the leap day
 * lands at the end of an era and the whole thing becomes arithmetic. Written
 * out rather than pulled in because a calendar library is not something this
 * capsule should link, and rather than approximated because a consensus is
 * accepted or refused on whether now is inside its window.
 */
fn days_from_civil(year: i64, month: i64, day: i64) -> Option<i64> {
    let shifted = year - i64::from(month <= 2);
    let era = if shifted >= 0 { shifted } else { shifted - 399 } / 400;
    let year_of_era = shifted - era * 400;
    let day_of_year = (153 * (month + if month > 2 { -3 } else { 9 }) + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    Some(era * 146_097 + day_of_era - 719_468)
}
