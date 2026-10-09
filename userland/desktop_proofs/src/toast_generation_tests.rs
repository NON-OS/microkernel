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

//! The toasts are part of the chrome's frame, and the shell repaints that
//! frame when the queue's generation moved past the one it was painted with
//! (`render/toasts.rs` sync_toast_layer). Every change to what the panel
//! shows must move the generation, and nothing else may, or a toast stays
//! on screen after it went, or the chrome is repainted every clock tick.

use crate::toast_state::toast::UptimeMs;
use crate::toast_state::toasts::ToastQueue;
use crate::toast_state::NotifyLevel;

/// The chrome as the shell paints it: it shows the queue as it was at the
/// generation it was last painted with.
struct Chrome {
    drawn: u32,
    shown: usize,
    paints: usize,
}

impl Chrome {
    fn sync(&mut self, q: &ToastQueue) {
        if self.drawn != q.generation() {
            self.drawn = q.generation();
            self.shown = q.iter_live().count();
            self.paints += 1;
        }
    }
}

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
