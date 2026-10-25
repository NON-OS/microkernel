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

//! Every change is painted, and nothing else.

use super::chrome::Chrome;
use crate::toast_state::toast::UptimeMs;
use crate::toast_state::toasts::ToastQueue;
use crate::toast_state::NotifyLevel;

#[test]
fn every_change_to_the_panel_is_painted() {
    let mut q = ToastQueue::new();
    let mut c = Chrome { drawn: q.generation(), shown: 0, paints: 0 };
    q.push(b"network connected", NotifyLevel::Info, UptimeMs(0));
    c.sync(&q);
    assert_eq!(c.shown, 1, "a pushed toast is drawn");
    q.push(b"saved", NotifyLevel::Info, UptimeMs(1000));
    c.sync(&q);
    assert_eq!(c.shown, 2);
    assert!(q.expire(UptimeMs(3000)), "the first toast's time is up");
    c.sync(&q);
    assert_eq!(c.shown, 1, "an expired toast leaves the chrome");
    q.clear();
    c.sync(&q);
    assert_eq!(c.shown, 0, "a dismissed panel leaves the chrome");
}

#[test]
fn a_tick_with_nothing_new_does_not_repaint() {
    let mut q = ToastQueue::new();
    let mut c = Chrome { drawn: q.generation(), shown: 0, paints: 0 };
    q.push(b"hello", NotifyLevel::Info, UptimeMs(0));
    c.sync(&q);
    for now in [100, 200, 300] {
        assert!(!q.expire(UptimeMs(now)));
        c.sync(&q);
    }
    q.clear();
    c.sync(&q);
    q.clear();
    c.sync(&q);
    assert_eq!(c.paints, 2, "one paint for the push, one for the dismissal");
}
