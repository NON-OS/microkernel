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

//! What the shell says of the battery.

use crate::battery_text::{label, Battery, LABEL_MAX};

#[test]
fn a_desktop_with_no_battery_says_no_battery() {
    let mut buf = [0u8; LABEL_MAX];
    let b = Battery::from_status(-19);
    assert_eq!(b, Battery::None);
    let n = label(b, &mut buf);
    assert_eq!(&buf[..n], b"No battery");
}

#[test]
fn an_unreadable_battery_says_so_in_words_not_zero_percent() {
    for rc in [-95, -5, -1, 101, i64::MIN] {
        let b = Battery::from_status(rc);
        assert_eq!(b, Battery::Unavailable, "rc {rc}");
        assert_eq!(b.percent(), None);
        let mut buf = [0u8; LABEL_MAX];
        let n = label(b, &mut buf);
        assert_eq!(&buf[..n], b"Battery status unavailable");
        assert!(!buf[..n].contains(&b'%'));
    }
}

#[test]
fn a_battery_reading_is_drawn_as_a_percent() {
    let mut buf = [0u8; LABEL_MAX];
    let n = label(Battery::from_status(57), &mut buf);
    assert_eq!(&buf[..n], b"57%");
    let n = label(Battery::from_status(7), &mut buf);
    assert_eq!(&buf[..n], b"7%");
    let n = label(Battery::from_status(100), &mut buf);
    assert_eq!(&buf[..n], b"100%");
    let n = label(Battery::from_status(0), &mut buf);
    assert_eq!(&buf[..n], b"0%", "a real 0 reading is still a reading");
}
