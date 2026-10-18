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

//! A held volume key keeps its notice up from the last press.

use crate::shell_system_keys::volume::is_volume_notice;
use crate::toast_state::toast::UptimeMs;
use crate::toast_state::toasts::ToastQueue;
use crate::toast_state::NotifyLevel;

#[test]
fn a_held_key_keeps_its_notice_up_from_the_last_press() {
    let mut q = ToastQueue::new();
    q.replace(b"Volume 50%", NotifyLevel::Info, UptimeMs(0), is_volume_notice);
    q.replace(b"Volume 55%", NotifyLevel::Info, UptimeMs(2_000), is_volume_notice);
    q.expire(UptimeMs(3_000));
    let live: Vec<_> = q.iter_live().map(|t| t.text[..t.len].to_vec()).collect();
    assert_eq!(live, [b"Volume 55%".to_vec()], "up 2.5 s from the last press, not the first");
    // The same level again (Volume Up held at full) puts it up again too.
    q.replace(b"Volume 55%", NotifyLevel::Info, UptimeMs(4_000), is_volume_notice);
    q.expire(UptimeMs(5_000));
    assert_eq!(q.iter_live().count(), 1);
}
