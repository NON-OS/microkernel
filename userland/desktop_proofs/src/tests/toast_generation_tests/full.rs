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

//! A full queue still moves on a push.

use crate::toast_state::toast::UptimeMs;
use crate::toast_state::toasts::ToastQueue;
use crate::toast_state::NotifyLevel;

#[test]
fn a_full_queue_still_moves_on_a_push() {
    let mut q = ToastQueue::new();
    for i in 0..3 {
        q.push(&[b'a' + i as u8], NotifyLevel::Info, UptimeMs(i));
    }
    let g = q.generation();
    q.push(b"y", NotifyLevel::Info, UptimeMs(10));
    assert_ne!(q.generation(), g, "the oldest toast was replaced: the panel changed");
}
