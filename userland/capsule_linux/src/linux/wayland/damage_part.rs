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

//! The rows a client said changed, read from it and laid into the window as
//! it is shown, and only that box damaged on screen.

use nonos_toolkit::decorations::frame_bg;

use crate::linux::guest::Guest;

use super::super::super::present::frame::{now::around_shape, paint::over};
use super::super::super::present::scaled;
use super::super::super::reshape::{reshape, Reshape};
use super::damage::Damage;

#[path = "damage_io.rs"]
mod damage_io;
#[path = "damage_lay.rs"]
mod damage_lay;

/// The bytes read showing `said` of `buffer` on the window `surface`; None
/// when it is shown whole: new, or its shape, scale, frame or layers changed.
pub fn part(guest: &mut Guest, surface: u32, buffer: u32, said: Damage) -> Option<u64> {
    let (pid, scene) = (guest.pid, &mut guest.scene);
    let on = scene.shown.as_ref().is_some_and(|s| s.surface == surface) && scene.out.is_some();
    /* Layers on it, or kept pixels of ones that went: a whole frame clears them. */
    let layered =
        !scene.subs.base.is_empty() || scene.subs.stacks.iter().any(|s| s.parent == surface);
    if said == Damage::Whole || !on || layered || scene.fit.due().is_some() {
        return None;
    }
    let src = scaled::window(scene, surface, buffer).ok()?;
    if src.raw.is_some() || reshape(scene.shape, src.shape) != Reshape::Same {
        return None;
    }
    let (w, h, stride) = src.shape;
    let (x0, y0, x1, y1) = match said.on(w, h) {
        Damage::None => return Some(0),
        Damage::Rect(x0, y0, x1, y1) => (x0, y0, x1, y1),
        Damage::Whole => return None,
    };
    let a = around_shape(scene, src.shape);
    let key = (a, scene.frame.hover, scene.keyboard_on.is_some());
    (!a.framed() || scene.frame.chrome == Some(key)).then_some(())?;
    let rows = damage_io::rows(pid, &src, (y0, y1))?;
    let (box_, put) = ((x0, 0, x1, y1 - y0), (stride, 0, y0));
    let ground = frame_bg();
    let blend = |px: u32| if px >> 24 == 0xFF { px } else { over(px, ground) };
    if !a.framed() {
        damage_lay::lay(scene.pixels.frame(src.bytes)?, put, &rows, stride, box_, blend);
        damage_io::tell(scene, (0, 0), (x0, y0, x1, y1));
        return Some(rows.len() as u64);
    }
    /* The guest's pixels kept whole for the frame's repaints, then the box laid. */
    damage_lay::lay(scene.frame.client_room(src.bytes)?, put, &rows, stride, box_, |px| px);
    let (cw, ch) = (x1.min(a.w.saturating_sub(a.cx)), y1.min(a.h.saturating_sub(a.cy)));
    let r = (x0.min(cw), y0.min(ch), cw, ch);
    let out = scene.pixels.frame(a.w as usize * a.h as usize * 4)?;
    damage_lay::lay(out, (a.w * 4, a.cx, a.cy), &scene.frame.client, stride, r, blend);
    damage_io::tell(scene, (a.cx, a.cy), r);
    Some(rows.len() as u64)
}
