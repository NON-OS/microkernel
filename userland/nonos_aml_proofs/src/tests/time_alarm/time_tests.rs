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

//! The 16-byte time: read back as written, and refused whole when it is
//! not 16 bytes, says invalid, or holds a field out of range.

use super::rig::time;
use crate::time_alarm::{RealTime, TZ_UNSPECIFIED};

#[test]
fn a_time_reads_back_as_written() {
    let t = RealTime::decode(&time(2026, 10, 12, 1)).unwrap();
    assert_eq!((t.year, t.month, t.day, t.hour, t.minute, t.second), (2026, 10, 12, 13, 45, 30));
    assert_eq!((t.millis, t.tz_minutes, t.daylight), (250, 60, 1));
    assert_eq!(RealTime::decode(&t.encode().unwrap()), Some(t));
    let utc = RealTime { tz_minutes: TZ_UNSPECIFIED, ..t };
    assert!(utc.encode().is_some(), "an unspecified zone");
}

#[test]
fn what_is_no_time() {
    assert_eq!(RealTime::decode(&time(2026, 10, 12, 0)), None, "says invalid");
    assert_eq!(RealTime::decode(&time(2026, 13, 12, 1)), None, "month 13");
    assert_eq!(RealTime::decode(&time(2026, 10, 0, 1)), None, "day 0");
    assert_eq!(RealTime::decode(&time(1899, 10, 12, 1)), None);
    assert_eq!(RealTime::decode(&time(2026, 10, 12, 1)[..15]), None, "15 bytes");
    let t = RealTime::decode(&time(2026, 10, 12, 1)).unwrap();
    for bad in [
        RealTime { hour: 24, ..t },
        RealTime { minute: 60, ..t },
        RealTime { second: 60, ..t },
        RealTime { millis: 1000, ..t },
        RealTime { tz_minutes: 1441, ..t },
    ] {
        assert_eq!(bad.encode(), None, "{bad:?}");
    }
}
