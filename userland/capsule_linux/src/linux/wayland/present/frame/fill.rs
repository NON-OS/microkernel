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
        compose(&mut scene.subs, surface, out, src.shape);
        return true;
    }
    let Some(client) = scene.frame.client_room(src.bytes) else { return false };
    if !take(pid, src, client) {
        return false;
    }
    compose(&mut scene.subs, surface, client, src.shape);
    let frame = &mut scene.frame;
    let key = (a, frame.hover, scene.keyboard_on.is_some());
    let chrome = frame.chrome != Some(key);
    frame.chrome = Some(key);
    let look = Look { title: &frame.title, hover: key.1, focused: key.2 };
    paint(out, a, &frame.client, src.shape, (&look, chrome), frame.quarters)
}
