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

//! Damage no pixel of which can show: a rectangle on a layer that lies
//! wholly inside an opaque layer drawn over it.
//!
//! The lock screen covers every window with the shell's chrome, opaque, and
//! a window under it that kept drawing (the Terminal's uptime row each
//! second) still had its damage composed, every layer under the curtain and
//! the curtain over them, for pixels that came out as the curtain's again.
//! Damage like that is not taken now. Nothing it covers is lost: what is
//! over it stays on the screen as it is, and the place it covers is damaged
//! again by whatever uncovers it: the cover moved, removed or dropped, or a
//! layer under it raised over it, damages that place (scene_submit.rs,
//! scene_remove.rs, visible_drop.rs, scene_raise.rs), and a present that is
//! no longer opaque damages the pixels it changed, which are all those that
//! now let anything through.
//!
//! A layer is opaque only while its last present said so; a layer whose
//! surface missed a paint is drawn by nothing and covers nothing.

use super::damage::Rect;
use super::damage_bound::by_owner;
use super::scene::{Layer, SceneTable};
use super::scene_submit::rect_of;

/// Composite rank, as scene/snapshot.rs draws: the band, then the raise
/// stamp inside it.
fn rank(l: &Layer) -> u64 {
    ((l.z as u64) << 32) | l.stack as u64
}

fn inside(r: Rect, c: Rect) -> bool {
    r.x >= c.x && r.y >= c.y && r.right() <= c.right() && r.bottom() <= c.bottom()
}

/// `r`, damage on `on`, lies wholly inside an opaque layer drawn over `on`.
pub fn hidden<'a>(layers: impl Iterator<Item = &'a Layer>, on: &Layer, r: Rect) -> bool {
    let over = |l: &&Layer| l.in_use && l.opaque && l.miss_count == 0 && rank(l) > rank(on);
    layers.filter(over).any(|l| inside(r, rect_of(l)))
}

/// The parts of `r` on `owner`'s layers (damage_bound.rs), less those
/// hidden under an opaque layer drawn over the layer they are on.
pub fn shown_by_owner(scene: &SceneTable, r: Rect, owner: u32) -> impl Iterator<Item = Rect> + '_ {
    by_owner(r, scene.layers(), owner)
        .filter(move |(l, p)| !hidden(scene.layers(), l, *p))
        .map(|(_, p)| p)
}

/// `r`, damage on `owner`'s layer in band `z` that shows `handle`, unless it
/// is hidden. Shown when no such layer is found: nothing is lost by
/// composing it.
pub fn shown_on(scene: &SceneTable, (owner, z, handle): (u32, u32, u64), r: Rect) -> Option<Rect> {
    let on = |l: &&Layer| l.owner_pid == owner && l.z == z && l.surface_handle == handle;
    match scene.layers().find(on) {
        Some(l) if hidden(scene.layers(), l, r) => None,
        _ => Some(r),
    }
}
