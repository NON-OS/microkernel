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

//! Nothing holds a notice past its lifetime.

use super::rig::{idle_loop, Screen};
use crate::toast_state::toast::{UptimeMs, TOAST_LIFETIME_MS};
use crate::toast_state::toasts::ToastQueue;
use crate::toast_state::NotifyLevel;

#[test]
fn a_notice_whose_app_ended_is_never_cleared_and_still_leaves() {
    // The app opened, its notice went up, and the app is gone: nothing ever
    // clears the notice or says anything more about that app.
    let mut q = ToastQueue::new();
    let mut s = Screen { synced: q.generation(), shown: 0 };
    q.push(b"Calculator opened", NotifyLevel::Info, UptimeMs(0));
    q.push(b"Calculator closed", NotifyLevel::Info, UptimeMs(300));
    s.synced = q.generation();
    s.shown = 2;
    let gone = idle_loop(&mut q, &mut s, 300, 60_000);
    assert_eq!(gone, Some(300 + TOAST_LIFETIME_MS));
    assert!(q.is_empty());
}

#[test]
fn a_repeated_notice_keeps_the_expiry_of_its_first_showing() {
    let mut q = ToastQueue::new();
    q.push(b"Terminal opened", NotifyLevel::Info, UptimeMs(0));
    let first = q.next_expiry();
    let generation = q.generation();
    // Said again every 100 ms while it is up.
    let mut now = 100;
    while now < TOAST_LIFETIME_MS {
        q.push(b"Terminal opened", NotifyLevel::Info, UptimeMs(now));
        assert_eq!(q.next_expiry(), first, "a repeat does not re-arm the timer");
        assert_eq!(q.iter_live().count(), 1, "a repeat is not a second row");
        now += 100;
    }
    assert_eq!(q.generation(), generation, "a repeat changes nothing on screen");
    assert!(q.expire(UptimeMs(TOAST_LIFETIME_MS)), "it leaves when its first showing is up");
    assert!(q.is_empty());
}

#[test]
fn a_notice_said_without_pause_never_outlives_its_lifetime() {
    // Quick open and close of the same app, said every 50 ms for a minute:
    // whatever is up at any moment was first shown less than a lifetime ago.
    let mut q = ToastQueue::new();
    let mut now = 0;
    while now < 60_000 {
        let text: &[u8] = if (now / 50) % 2 == 0 { b"Files opened" } else { b"Files closed" };
        q.push(text, NotifyLevel::Info, UptimeMs(now));
        q.expire(UptimeMs(now));
        for t in q.iter_live() {
            assert!(now - t.shown_at_ms < TOAST_LIFETIME_MS, "a notice outlived its lifetime");
        }
        now += 50;
    }
}
