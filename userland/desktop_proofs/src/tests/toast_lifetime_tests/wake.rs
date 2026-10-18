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

//! The loop wakes in time, and the one notice held on purpose.

use super::rig::RECV_BLOCK;
use crate::shell_tick::{wait_ms, wake_at, TICK_MS};
use crate::toast_state::toast::{UptimeMs, TOAST_HELD_MS};
use crate::toast_state::toasts::ToastQueue;
use crate::toast_state::NotifyLevel;

#[test]
fn the_wake_is_never_later_than_the_tick_or_the_expiry() {
    assert_eq!(wake_at(0, None), TICK_MS);
    assert_eq!(wake_at(0, Some(300)), 300);
    assert_eq!(wake_at(0, Some(5000)), TICK_MS);
    assert_eq!(wait_ms(0, 300, RECV_BLOCK), 300);
    assert_eq!(wait_ms(0, 5000, RECV_BLOCK), RECV_BLOCK);
    assert_eq!(wait_ms(400, 300, RECV_BLOCK), 1, "a wake already due waits as little as it can");
}

#[test]
fn the_store_warning_is_held_by_design_bounded_and_dismissed_by_a_press() {
    let mut q = ToastQueue::new();
    q.push_held(b"The capsule store could not be read", NotifyLevel::Error, UptimeMs(0));
    assert!(!q.expire(UptimeMs(TOAST_HELD_MS - 1)), "held up longer than a transient notice");
    assert!(q.expire(UptimeMs(TOAST_HELD_MS)), "but not for ever");

    let mut q = ToastQueue::new();
    let g = q.generation();
    q.push_held(b"The capsule store could not be read", NotifyLevel::Error, UptimeMs(0));
    q.clear();
    assert!(q.is_empty(), "a press on the panel dismisses it");
    assert_ne!(q.generation(), g);
}
