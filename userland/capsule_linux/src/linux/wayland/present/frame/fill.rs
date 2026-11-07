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

//! A buffer made the window's pixels: the guest's own, with its subsurfaces
//! and popups, then the frame around them.

use super::super::super::commit::launch_clock::laid;
use super::super::super::frame_cost::box_bytes;
use super::super::super::scene::Scene;
use super::super::super::subsurface::compose::compose;
use super::super::source::{take, Source};
use super::geometry::Around;
use super::paint::{paint, Look};

/// How many bytes the window's pixels take with frame `a`.
pub fn window_bytes(a: Around, src: &Source) -> usize {
    match a.framed() {
        true => (a.w as usize) * (a.h as usize) * 4,
        false => src.bytes,
    }
}

/// `src` on `surface`, read from guest `pid`, into `out` with frame `a`.
pub fn fill(
    scene: &mut Scene,
    (pid, surface): (u32, u32),
    src: &Source,
    out: &mut [u8],
    a: Around,
) -> bool {
    if !a.framed() {
        let Some(out) = out.get_mut(..src.bytes) else { return false };
        if !take(pid, src, out) {
            return false;
        }
        let e = (scene.win.frame.ratio, scene.win.origin);
        compose(&scene.subs, &mut scene.win.base, surface, out, src.shape, e);
        opaque(out, src.shape);
        return true;
    }
    let e = (scene.win.frame.ratio, scene.win.origin);
    // Of no shape until it is whole again (state.rs).
    scene.win.frame.client_shape = None;
    let Some(client) = scene.win.frame.client_room(src.bytes) else { return false };
    if !take(pid, src, client) {
        return false;
    }
    compose(&scene.subs, &mut scene.win.base, surface, client, src.shape, e);
    let frame = &mut scene.win.frame;
    frame.client_shape = Some(src.shape);
    let key = (a, frame.hover, scene.win.focused);
    let chrome = frame.chrome != Some(key);
    frame.chrome = Some(key);
    let look = Look { title: &frame.title, hover: key.1, focused: key.2 };
    let (cw, ch) =
        (src.shape.0.min(a.w.saturating_sub(a.cx)), src.shape.1.min(a.h.saturating_sub(a.cy)));
    let painted = paint(out, a, &frame.client, src.shape, (&look, chrome), frame.quarters);
    if painted {
        laid(box_bytes(cw, ch));
    }
    painted
}

/// Every pixel of a frameless window made opaque over the frame's ground: the
/// compositor blends, so a guest clearing to transparent shows the desktop.
fn opaque(out: &mut [u8], (w, h, stride): (u32, u32, u32)) {
    let ground = nonos_toolkit::decorations::frame_bg();
    let n = w as usize * 4;
    for row in out.chunks_exact_mut(stride as usize).take(h as usize) {
        let n = n.min(row.len());
        nonos_pixel_rows::flatten(&mut row[..n], ground);
    }
}
