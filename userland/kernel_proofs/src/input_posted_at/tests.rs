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

use super::posted_at::posted_ns;

#[test]
fn an_event_is_dated_in_nanoseconds_on_the_uptime_clock() {
    assert_eq!(posted_ns(0), 0, "at boot");
    assert_eq!(posted_ns(1), 1_000_000);
    assert_eq!(posted_ns(400), 400_000_000, "a double click's whole window");
    let a = posted_ns(10_000);
    let b = posted_ns(10_310);
    assert_eq!((b - a) / 1_000_000, 310, "two presses 310 ms apart read 310 ms apart");
}

#[test]
fn a_later_post_is_never_dated_earlier() {
    let mut last = 0;
    for ms in (0..u64::MAX / 1_000_000 + 4).step_by(1 << 40).chain([u64::MAX - 1, u64::MAX]) {
        let ns = posted_ns(ms);
        assert!(ns >= last, "{ms} ms");
        last = ns;
    }
}

#[test]
fn a_time_past_the_last_nanosecond_saturates_and_does_not_wrap() {
    let edge = u64::MAX / 1_000_000;
    assert_eq!(posted_ns(edge), edge * 1_000_000);
    assert_eq!(posted_ns(edge + 1), u64::MAX);
    assert_eq!(posted_ns(u64::MAX), u64::MAX);
}
