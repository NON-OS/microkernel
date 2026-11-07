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

//! The window's frame with its subsurfaces laid on it. The window's own
//! pixels are kept beside it while a subsurface shows on it, so a commit
//! that changes only a subsurface repaints from them (repaint), the
//! toplevel's buffer having gone back to the client at its own commit. A
//! window whose subsurfaces show nothing keeps none, and is shown as one
//! with no subsurface is: in part, or from its pin.

use alloc::vec::Vec;

use crate::linux::guest::Guest;

use super::super::present::draw::{draw, whole, Drew};
use super::super::present::frame::now::around_shape;
use super::super::present::frame::paint::{paint as framed, Look};
use super::super::present::{scale_math::point_px, surface::say};
use super::super::reshape::Shape;
use super::super::scene::Scene;
use super::blend::{lay, Plane};
use super::order::layers;
use super::state::Subs;

/// Lay the subsurfaces placed on `root` onto its `frame`, shaped `shape`, at
/// the window's ratio and corner `e`, keeping the frame as it was in `base`
/// for a later repaint.
pub fn compose(
    subs: &Subs,
    base: &mut Vec<u8>,
    root: u32,
    frame: &mut [u8],
    shape: Shape,
    e: (u32, (i32, i32)),
) {
    if !subs.shown_on(root) {
        *base = Vec::new();
        return;
    }
    base.clear();
    if base.try_reserve_exact(frame.len()).is_err() {
        say("[WAYLAND] no room to keep the window's pixels for its subsurfaces\n".into());
    } else {
        base.extend_from_slice(frame);
    }
    paint(subs, root, frame, shape, e);
}

/// Paint the window again from its kept pixels, as a subsurface changed.
/// A window shown from its pin keeps only the frame's ground in its own
/// buffers, under the client's: it goes back to them first, drawn whole
/// with its subsurfaces laid on it (present/pinned/show.rs give_back).
pub fn repaint(guest: &mut Guest) -> Drew {
    if guest.scene.win.content.is_some() {
        return super::super::present::pinned::show::give_back(guest);
    }
    let scene = &mut guest.scene;
    let (Some(shown), Some(shape)) = (scene.win.shown.as_ref(), scene.win.shape) else {
        return Drew::Nothing;
    };
    let root = shown.surface;
    let (e, a) = ((scene.win.frame.ratio, scene.win.origin), around_shape(scene, shape));
    let len = shape.2 as usize * shape.1 as usize;
    if scene.win.base.is_empty() && scene.subs.shown_on(root) {
        seed(scene, len, a.framed());
    }
    // Kept for this shape, not one a refused show left behind.
    if scene.win.base.len() != len || len == 0 {
        return Drew::Nothing;
    }
    // Drawn only at the layout the buffers are laid at; a frame of another
    // shape waits for the commit that takes new buffers (present/show.rs).
    if scene.win.bufs.shape() != a.laid(shape) {
        return Drew::Nothing;
    }
    let all = whole((a.w, a.h));
    let drew = if a.framed() {
        // The window's own pixels are kept in the client's shape; the frame
        // round them is wider, so they are laid into it as a show lays them
        // (frame/fill.rs), never copied over it row for row.
        let Some(client) = scene.win.frame.client_room(len) else { return Drew::Nothing };
        client.copy_from_slice(&scene.win.base);
        paint(&scene.subs, root, client, shape, e);
        scene.win.frame.client_shape = Some(shape);
        let key = (a, scene.win.frame.hover, scene.win.focused);
        let chrome = scene.win.frame.chrome != Some(key);
        let (cw, ch) =
            (shape.0.min(a.w.saturating_sub(a.cx)), shape.1.min(a.h.saturating_sub(a.cy)));
        let cover = if chrome { all } else { (a.cx, a.cy, cw, ch) };
        let bytes = a.w as usize * a.h as usize * 4;
        let drew = draw(scene, bytes, cover, |scene, out| {
            let frame = &scene.win.frame;
            let look = Look { title: &frame.title, hover: key.1, focused: key.2 };
            framed(out, a, &frame.client, shape, (&look, chrome), frame.quarters).then_some(all)
        });
        if drew == Drew::Shown {
            scene.win.frame.chrome = Some(key);
        }
        drew
    } else {
        draw(scene, len, all, |scene, frame| {
            frame.copy_from_slice(&scene.win.base);
            paint(&scene.subs, root, frame, shape, e);
            flatten(frame, shape);
            Some(all)
        })
    };
    if drew == Drew::Shown {
        super::super::present::owed::paid_repaint(&mut scene.win);
    }
    drew
}

/// The first layer on a window that has not drawn since: its base is the
/// guest's pixels it last showed. With no layer on it they are the guest's
/// own, whole (frame/fill.rs, damage_part.rs): a framed window's beside the
/// frame, an unframed one's as shown, over the ground, which a layer laid
/// over them leaves as composing first and grounding after would.
fn seed(scene: &mut Scene, len: usize, framed: bool) {
    let win = &mut scene.win;
    let shown = match framed {
        // Only pixels of the window's shape (frame/state.rs).
        true => win.frame.client.get(..len).filter(|_| win.shape.is_some_and(|s| win.frame.holds(s))),
        false => win.bufs.shown_frame(len),
    };
    let Some(shown) = shown else { return };
    if win.base.try_reserve_exact(len).is_err() {
        say("[WAYLAND] no room to keep the window's pixels for its subsurfaces\n".into());
        return;
    }
    win.base.extend_from_slice(shown);
}

/// An unframed window's translucent pixels onto the frame's ground, as a
/// show leaves them (frame/fill.rs).
fn flatten(out: &mut [u8], (w, h, stride): Shape) {
    let ground = nonos_toolkit::decorations::frame_bg();
    let n = w as usize * 4;
    for row in out.chunks_exact_mut(stride as usize).take(h as usize) {
        let n = n.min(row.len());
        nonos_pixel_rows::flatten(&mut row[..n], ground);
    }
}

/// Each subsurface on `root` laid where it is from the window's corner: the
/// surface's point `o` (its window geometry's, present/crop.rs) at ratio `e`.
fn paint(
    subs: &Subs,
    root: u32,
    frame: &mut [u8],
    (w, h, stride): Shape,
    (e, o): (u32, (i32, i32)),
) {
    let at = |id| subs.of(id).filter(|s| s.shown.is_some()).map(|s| point_px(s.at, e));
    let (below, above) = layers(&subs.stacks, root, &at);
    let (ox, oy) = point_px(o, e);
    let from = |(x, y): (i32, i32)| (x.saturating_sub(ox), y.saturating_sub(oy));
    let plane = Plane { w, h, stride };
    // Beneath the window, nearest first; above it, lowest first.
    let lays = below.iter().rev().map(|l| (l, false)).chain(above.iter().map(|l| (l, true)));
    for (&(id, xy), over) in lays {
        if let Some(k) = subs.of(id).and_then(|s| s.shown.as_ref()) {
            lay(frame, plane, &k.px, k.plane, from(xy), over);
        }
    }
}
