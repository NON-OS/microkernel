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

//! Every draw into a window, the one way: a buffer the compositor is not
//! reading taken and brought up to the picture shown, drawn, then
//! presented with what changed (OP_PRESENT_LAYER, naming the layer by the
//! buffer it shows). The compositor reads the new buffer only once it is
//! drawn whole; the one it showed is released after the next frame.

use nonos_app_skeleton::clients::compositor::{present_layer, present_layer_parts};
use nonos_app_skeleton::discover::lookup_port;
use nonos_app_skeleton::swap::{on_layer, parts_block, with_parts, MAX_PARTS};

use super::super::scene::Scene;
use super::super::scene_buffers::Patience;
use super::super::swap_copy::{clip, Area};
use super::owed::{owe, Owed};

/// What a draw came to.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Drew {
    /// Presented: the layer shows it.
    Shown,
    /// Nothing changed, or the draw could not be made (said where it
    /// failed): the picture shown stays as it was.
    Nothing,
    /// No buffer was free, or the compositor refused the present: the
    /// window is owed the draw (owed.rs).
    Owed,
    /// Shown from the client's buffer itself, pinned (pinned/): the window
    /// keeps the buffer, given back once the compositor lets it go.
    Held,
}

/// The whole of a `w` by `h` window.
pub fn whole((w, h): (u32, u32)) -> Area {
    (0, 0, w, h)
}

/// `f` draws into the window's `bytes` long buffer, every pixel of `cover`
/// and what else it likes, and answers the box it changed, in buffer
/// pixels; the box presented. None from `f` presents nothing.
pub fn draw(
    scene: &mut Scene,
    bytes: usize,
    cover: Area,
    f: impl FnOnce(&mut Scene, &mut [u8]) -> Option<Area>,
) -> Drew {
    draw_as(scene, bytes, cover, Patience::Fence, f)
}

/// `draw`, waiting for a free buffer only as `patience` allows.
pub fn draw_as(
    scene: &mut Scene,
    bytes: usize,
    cover: Area,
    patience: Patience,
    f: impl FnOnce(&mut Scene, &mut [u8]) -> Option<Area>,
) -> Drew {
    draw_told(scene, (bytes, cover, patience), &[], f)
}

/// `draw` where `f` draws every pixel of each of `parts` and answers the
/// box round them: the compositor is told the parts apart inside that box
/// (OP_PRESENT_LAYER_PARTS), so it composes them and not the pixels between.
pub fn draw_parts(
    scene: &mut Scene,
    bytes: usize,
    parts: &[Area],
    f: impl FnOnce(&mut Scene, &mut [u8]) -> Option<Area>,
) -> Drew {
    let size = |&(_, _, w, h): &Area| u64::from(w) * u64::from(h);
    let cover = parts.iter().copied().max_by_key(size).unwrap_or_default();
    draw_told(scene, (bytes, cover, Patience::Fence), parts, f)
}

fn draw_told(
    scene: &mut Scene,
    (bytes, cover, patience): (usize, Area, Patience),
    parts: &[Area],
    f: impl FnOnce(&mut Scene, &mut [u8]) -> Option<Area>,
) -> Drew {
    let (Some(on), Some(port)) = (scene.win.out, lookup_port(b"compositor")) else {
        return Drew::Nothing;
    };
    if !scene.win.bufs.fits(bytes) {
        return Drew::Nothing;
    }
    let Some(i) = scene.win.bufs.acquire(cover, patience) else {
        owe(&mut scene.win, Owed::Repaint);
        return Drew::Owed;
    };
    let mut px = scene.win.bufs.take(i);
    let changed = px.frame(bytes).and_then(|out| f(scene, out));
    scene.win.bufs.put(i, px);
    let (w, h, _) = scene.win.bufs.shape();
    let shown = changed.and_then(|a| clip(a, (w, h)));
    let Some((a, pending)) = shown.and_then(|a| Some((a, scene.win.bufs.present(i, a)?))) else {
        undrawn(scene, i);
        return Drew::Nothing;
    };
    let body = on_layer(&pending.body, on);
    let rid = scene.next_serial();
    let taken = match apart(parts, a) {
        Some((list, n)) => parts_block(&list[..n])
            .is_some_and(|b| present_layer_parts(port, rid, &with_parts(&body, &b)).is_ok()),
        None => present_layer(port, rid, &body).is_ok(),
    };
    if !scene.win.bufs.settle(pending, taken) {
        // Given back as settled; what it held is not the picture.
        scene.win.frame.chrome = None;
        owe(&mut scene.win, Owed::Repaint);
        return Drew::Owed;
    }
    scene.win.out = Some(scene.win.bufs.handle(i));
    Drew::Shown
}

/// Buffer `i` given back unpresented. Whatever the draw wrote of the frame
/// is not on screen, so the next paints the frame again.
fn undrawn(scene: &mut Scene, i: usize) {
    scene.win.bufs.cancel(i);
    scene.win.frame.chrome = None;
}

/// `parts` cut to `a`, the box presented, those left empty dropped, when
/// two or more are left: each lies inside the box, as the compositor holds.
fn apart(parts: &[Area], a: Area) -> Option<([Area; MAX_PARTS], usize)> {
    let mut out = [(0, 0, 0, 0); MAX_PARTS];
    let mut n = 0;
    for p in parts.iter().filter_map(|&p| inside(p, a)) {
        *out.get_mut(n)? = p;
        n += 1;
    }
    (n >= 2).then_some((out, n))
}

/// The pixels `p` and `a` share.
fn inside((px, py, pw, ph): Area, (ax, ay, aw, ah): Area) -> Option<Area> {
    let (x0, y0) = (px.max(ax), py.max(ay));
    let x1 = px.saturating_add(pw).min(ax.saturating_add(aw));
    let y1 = py.saturating_add(ph).min(ay.saturating_add(ah));
    (x0 < x1 && y0 < y1).then(|| (x0, y0, x1 - x0, y1 - y0))
}
