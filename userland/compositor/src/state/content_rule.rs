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

//! What a change of a layer's content repaints, pure; compositor_proofs
//! holds it. A first surface, none, or one at another corner or drawn
//! another way, repaints the whole layer, where the old was and the new is.
//! The same surface drawn again, or its owner's next buffer in the old
//! one's place (a swapchain's), repaints the box of it that changed, on the
//! canvas and cut to the layer, as a present of a layer's own surface does
//! (present_rule.rs).

use super::damage::Rect;
use super::scene::{Content, Layer};

/// Whether `sender` may have its layer draw a surface the kernel says
/// `answerable` answers for: its own, or one it pinned from a guest of its.
/// One client naming another's surface (a handle is a slot and a small
/// counter, easy to guess) would show that surface as its own window.
pub fn may_show(answerable: Option<u32>, sender: u32) -> bool {
    sender != 0 && answerable == Some(sender)
}

/// `was`: the layer before the change. `area`: the box of the content that
/// changed, in its own pixels.
pub fn repaint(was: &Layer, now: Content, area: Rect) -> Option<Rect> {
    let on = Rect { x: was.x, y: was.y, width: was.width, height: was.height };
    if was.content != now && !swapped(was.content, now) {
        return (on.width != 0 && on.height != 0).then_some(on);
    }
    if now.handle == 0 {
        return None;
    }
    let x = was.x.saturating_add(now.x).saturating_add(area.x);
    let y = was.y.saturating_add(now.y).saturating_add(area.y);
    Rect { x, y, width: area.width, height: area.height }.intersect(on)
}

/// `now` takes `was`'s place: another surface where a surface was drawn, at
/// the same corner and drawn the same way.
fn swapped(was: Content, now: Content) -> bool {
    was.handle != 0 && now.handle != 0 && Content { handle: now.handle, ..was } == now
}
