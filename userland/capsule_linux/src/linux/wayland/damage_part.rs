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

//! The box a client said changed, read from it into the window as it is
//! shown, and only that box damaged on screen. A buffer shown at its own
//! size has the box read straight into the picture it lands in: the
//! window's buffer when it is unframed, the kept client pixels when it is
//! framed. Read whole rows into a scratch first and laid from there, a
//! scroll moved its pixels three times and read every row's pixels beside
//! the box.

use nonos_toolkit::decorations::frame_bg;

use crate::linux::guest::Guest;

use super::super::super::frame_cost::corner_bytes;
use super::super::super::present::draw::{draw, Drew};
use super::super::super::present::frame::geometry::Around;
use super::super::super::present::frame::now::around_shape;
use super::super::super::present::scaled;
use super::super::super::reshape::{reshape, Reshape};
use super::super::super::scene::Scene;
use super::damage::Damage;
use super::damage_boxes::Boxes;
use super::launch_clock::laid;
use damage_io::Corners;

#[path = "damage_apart.rs"]
mod damage_apart;
#[path = "damage_io.rs"]
mod damage_io;
#[path = "damage_lay.rs"]
mod damage_lay;
#[path = "damage_place.rs"]
mod damage_place;
#[path = "damage_scale.rs"]
mod damage_scale;
#[path = "damage_scaled.rs"]
mod damage_scaled;

/// The bytes read showing `said` of `buffer` on the window `surface`, and
/// what the draw came to; None when it is shown whole: new, its shape,
/// scale, frame or layers changed, or the window owes a draw (owed.rs).
pub fn part(
    guest: &mut Guest,
    surface: u32,
    buffer: u32,
    (said, boxes): (Damage, &Boxes),
) -> Option<(u64, Drew)> {
    let (pid, scene, spans) = (guest.pid, &mut guest.scene, &guest.memfd_spans);
    let on =
        scene.win.shown.as_ref().is_some_and(|s| s.surface == surface) && scene.win.out.is_some();
    /* Layers on it, or kept pixels of ones that went: a whole frame clears them. */
    let layered = !scene.win.base.is_empty() || scene.subs.shown_on(surface);
    let owes = scene.win.owing.is_some();
    if said == Damage::Whole || !on || layered || owes || scene.win.fit.due().is_some() {
        return None;
    }
    let src = scaled::window(scene, spans, surface, buffer).ok()?;
    if reshape(scene.win.shape, src.shape) != Reshape::Same || scene.win.origin != src.origin {
        return None;
    }
    let a = around_shape(scene, src.shape);
    let key = (a, scene.win.frame.hover, scene.win.focused);
    (!a.framed() || scene.win.frame.chrome == Some(key)).then_some(())?;
    // Laid at another layout than the buffers are, it is shown whole, and
    // that show takes buffers of the frame's shape (present/show.rs).
    (scene.win.bufs.shape() == a.laid(src.shape)).then_some(())?;
    let said = said.moved(src.cut);
    if src.raw.is_some() {
        /* At the size shown: a buffer of another size is resampled in the box only. */
        return damage_scaled::part(pid, scene, &src, a, said);
    }
    let b = match said.on(src.shape.0, src.shape.1) {
        Damage::None => return Some((0, Drew::Nothing)),
        Damage::Rect(x0, y0, x1, y1) => (x0, y0, x1, y1),
        Damage::Whole => return None,
    };
    let plan = damage_io::plan(src.shape, src.read, b)?;
    // Two boxes or more said apart inside it: each read and shown alone.
    match (a.framed(), boxes.apart(src.cut, b)) {
        (false, None) => damage_place::unframed(pid, scene, &src, plan, b),
        (true, None) => damage_place::framed(pid, scene, &src, a, (plan, b)),
        (false, Some(parts)) => damage_apart::unframed(pid, scene, &src, (b, &parts)),
        (true, Some(parts)) => damage_apart::framed(pid, scene, &src, a, (b, &parts)),
    }
}

/// Box `b` of the kept client pixels, `stride` bytes a row, laid into the
/// framed window `a` and presented.
fn show_box(scene: &mut Scene, a: Around, stride: u32, b: Corners) -> Drew {
    let (r, changed) = placed(a, b);
    let bytes = a.w as usize * a.h as usize * 4;
    let ground = Some(frame_bg());
    draw(scene, bytes, changed, |scene, out| {
        let client = &scene.win.frame.client;
        let ok = damage_lay::lay(out, (a.w * 4, a.cx, a.cy), client, stride, r, ground);
        counted(ok, r).then_some(changed)
    })
}

/// Box `b` of the client's pixels cut to the framed window `a`'s content,
/// and where it lands in the window, as x, y, width and height.
fn placed(a: Around, (x0, y0, x1, y1): Corners) -> (Corners, (u32, u32, u32, u32)) {
    let (cw, ch) = (x1.min(a.w.saturating_sub(a.cx)), y1.min(a.h.saturating_sub(a.cy)));
    let r = (x0.min(cw), y0.min(ch), cw, ch);
    (r, (a.cx + r.0, a.cy + r.1, r.2 - r.0, r.3 - r.1))
}

/// `ok`, with box `b` counted as laid when it is (frame_cost.rs): a lay
/// that stopped part way counts nothing.
fn counted(ok: bool, b: Corners) -> bool {
    if ok {
        laid(corner_bytes(b));
    }
    ok
}
