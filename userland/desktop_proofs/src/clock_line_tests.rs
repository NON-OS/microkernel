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

use crate::clock_line::{line, Moment, LINE_MAX};

fn at(hour: u8, minute: u8, h24: bool) -> String {
    let m = Moment { year: 2026, month: 10, day: 3, hour, minute };
    let mut buf = [0u8; LINE_MAX];
    let n = line(&mut buf, &m, h24);
    String::from_utf8(buf[..n].to_vec()).unwrap()
}

#[test]
fn a_24_hour_clock_shows_the_hour_as_it_is() {
    assert_eq!(at(19, 42, true), "Sat 03 Oct  19:42");
    assert_eq!(at(0, 5, true), "Sat 03 Oct  00:05");
}

/* The morning and the evening no longer read the same. */
#[test]
fn a_12_hour_clock_says_am_or_pm() {
    assert_eq!(at(7, 42, false), "Sat 03 Oct   7:42 AM");
    assert_eq!(at(19, 42, false), "Sat 03 Oct   7:42 PM");
    assert_eq!(at(0, 0, false), "Sat 03 Oct  12:00 AM", "midnight is 12 AM");
    assert_eq!(at(12, 30, false), "Sat 03 Oct  12:30 PM", "noon is 12 PM");
    assert_eq!(at(11, 59, false), "Sat 03 Oct  11:59 AM");
}

#[test]
fn the_weekday_follows_the_date() {
    let mut buf = [0u8; LINE_MAX];
    let leap = Moment { year: 2024, month: 2, day: 29, hour: 9, minute: 0 };
    let n = line(&mut buf, &leap, true);
    assert_eq!(&buf[..n], b"Thu 29 Feb  09:00");
    let first = Moment { year: 2000, month: 1, day: 1, hour: 9, minute: 0 };
    let n = line(&mut buf, &first, true);
    assert_eq!(&buf[..3], b"Sat");
    assert_eq!(n, 17);
}
