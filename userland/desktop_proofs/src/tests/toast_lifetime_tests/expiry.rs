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

//! A notice expires and leaves the screen with no other input.

use super::rig::{idle_loop, Screen};
use crate::toast_state::toast::{UptimeMs, TOAST_LIFETIME_MS};
use crate::toast_state::toasts::ToastQueue;
use crate::toast_state::NotifyLevel;

#[test]
fn the_lifetime_is_short() {
    const { assert!(TOAST_LIFETIME_MS >= 2000 && TOAST_LIFETIME_MS <= 3000) };
}

#[test]
fn an_expired_toast_is_no_longer_painted_with_no_other_input() {
    let mut q = ToastQueue::new();
    let mut s = Screen { synced: q.generation(), shown: 0 };
    // A launch at uptime 1234, off any tick boundary.
    q.push(b"Terminal opened", NotifyLevel::Info, UptimeMs(1234));
    s.synced = q.generation();
    s.shown = 1;
    let gone = idle_loop(&mut q, &mut s, 1234, 1234 + 10_000);
    assert_eq!(
        gone,
        Some(1234 + TOAST_LIFETIME_MS),
        "the loop wakes for the expiry itself, not on the next tick or repaint"
    );
    assert_eq!(s.shown, 0);
}

#[test]
fn every_transient_notice_leaves_within_three_seconds() {
    let words: [(&[u8], NotifyLevel); 8] = [
        (b"Terminal opened", NotifyLevel::Info),
        (b"Terminal closed", NotifyLevel::Info),
        (b"opening a new window", NotifyLevel::Info),
        (b"window limit reached, focusing", NotifyLevel::Warn),
        (b"Files did not open", NotifyLevel::Error),
        (b"network connected", NotifyLevel::Info),
        (b"saved /home/notes.txt", NotifyLevel::Info),
        (b"clipboard unavailable", NotifyLevel::Warn),
    ];
    for (i, (text, level)) in words.into_iter().enumerate() {
        let mut q = ToastQueue::new();
        let mut s = Screen { synced: q.generation(), shown: 0 };
        let at = 500 + 137 * i as i64;
        q.push(text, level, UptimeMs(at));
        s.synced = q.generation();
        s.shown = 1;
        let gone =
            idle_loop(&mut q, &mut s, at, at + 10_000).expect("every notice leaves on its own");
        assert!(gone - at <= 3000, "a notice stayed {} ms", gone - at);
    }
}
