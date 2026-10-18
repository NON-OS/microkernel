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

//! The clock and the drain cannot hold a notice either.

use super::rig::{idle_loop, Screen};
use crate::shell_tick::{wake_at, wake_due};
use crate::toast_state::toast::UptimeMs;
use crate::toast_state::toasts::ToastQueue;
use crate::toast_state::NotifyLevel;

#[test]
fn a_clock_reading_behind_the_showing_does_not_hold_a_notice() {
    // The wall clock steps back when NTP corrects an RTC kept in local time;
    // a toast stamped before the step stayed up for the whole step. The
    // queue takes uptime only, and a reading behind a toast ends it.
    let mut q = ToastQueue::new();
    q.push(b"Terminal opened", NotifyLevel::Info, UptimeMs(7_200_000));
    assert!(q.expire(UptimeMs(5_000)), "a clock that went back does not hold it up");
    assert!(q.is_empty());
}

#[test]
fn a_clock_that_never_advances_is_not_the_one_toasts_use() {
    // The wall clock reads as the same error code (-61) on every call until
    // the RTC is read; a toast timed on it was never let go. On uptime, which
    // the loop's own wait advances, it leaves.
    let mut q = ToastQueue::new();
    let mut s = Screen { synced: q.generation(), shown: 0 };
    q.push(b"Terminal opened", NotifyLevel::Info, UptimeMs(0));
    s.synced = q.generation();
    s.shown = 1;
    assert!(idle_loop(&mut q, &mut s, 0, 5_000).is_some());
}

#[test]
fn a_stream_of_messages_does_not_hold_the_loop_in_the_drain() {
    // A pointer moving, or an app redrawing, sends faster than any wait; the
    // drain still goes back to the loop once its wake is due.
    let mut q = ToastQueue::new();
    q.push(b"Files closed", NotifyLevel::Info, UptimeMs(0));
    let wake = wake_at(0, q.next_expiry().map(|t| t.0));
    let mut now = 0;
    let mut handled = 0;
    while !wake_due(now, wake) {
        now += 4;
        handled += 1;
        assert!(handled < 100_000, "the drain never gave the loop back");
    }
    assert!(now <= wake + 4);
}
