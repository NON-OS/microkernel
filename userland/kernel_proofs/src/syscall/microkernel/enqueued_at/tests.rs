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

use super::rule::enqueued_uptime;

#[test]
fn a_message_queued_earlier_is_dated_back_by_its_age_in_uptime() {
    // Uptime 50_000 ms; the stamp clock, 1_700_000_000_000; queued 1_250 ms ago.
    let (uptime, stamp) = (50_000, 1_700_000_000_000);
    assert_eq!(enqueued_uptime(uptime, stamp, stamp - 1_250), 48_750);
    assert_eq!(enqueued_uptime(uptime, stamp, stamp), uptime, "queued now");
}

#[test]
fn neither_clock_running_backwards_breaks_it() {
    let (uptime, stamp) = (50_000, 1_700_000_000_000);
    assert_eq!(enqueued_uptime(uptime, stamp, stamp + 5), uptime, "a stamp ahead reads as now");
    assert_eq!(enqueued_uptime(uptime, stamp, 0), 0, "older than the boot is the boot");
    assert_eq!(enqueued_uptime(0, 0, 0), 0);
}

#[test]
fn two_messages_keep_their_order_and_their_gap() {
    let (uptime, stamp) = (90_000, 7_000_000);
    let first = enqueued_uptime(uptime, stamp, stamp - 400);
    let second = enqueued_uptime(uptime, stamp, stamp - 150);
    assert!(first < second);
    assert_eq!(second - first, 250);
}
