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

//! The box a client said changed of a buffer shown at its own size, read
//! from the guest into the picture it lands in (damage_box.rs plans the
//! reads): the window's buffer when it is unframed, the kept client pixels
//! when it is framed.

use nonos_toolkit::decorations::frame_bg;

use super::super::super::super::present::draw::{draw, Drew};
use super::super::super::super::present::frame::geometry::Around;
use super::super::super::super::present::source::Source;
use super::super::super::super::scene::Scene;
use super::damage_io::{self, Corners, Reads};
use super::show_box;

/// Box `b` read into the window's buffer, flattened onto the frame's
/// ground as the compositor blends; None when the read failed, and the
/// window is shown whole.
pub fn unframed(
    pid: u32,
    scene: &mut Scene,
    src: &Source,
    plan: Reads,
    b: Corners,
) -> Option<(u64, Drew)> {
    let (x0, y0, x1, y1) = b;
    let changed = (x0, y0, x1 - x0, y1 - y0);
    let ground = frame_bg();
    let mut read = None;
    let drew = draw(scene, src.bytes, changed, |_, out| {
        let flat = |px: &mut [u8]| nonos_pixel_rows::flatten(px, ground);
        let got = damage_io::read_in(pid, src, plan, out, flat);
        read = Some(got);
        got.map(|_| changed)
    });
    match read {
        Some(Some(n)) => Some((n, drew)),
        // Owed before a buffer was taken: nothing read.
        None => Some((0, drew)),
        Some(None) => None,
    }
}

/*
 * Box `b` read into the guest's pixels kept whole for the frame's repaints,
 * then laid into the window. Only over a picture of this shape: the rest of
 * the window is what they hold, so pixels of another are shown whole.
 */
pub fn framed(
    pid: u32,
    scene: &mut Scene,
    src: &Source,
    a: Around,
    (plan, b): (Reads, Corners),
) -> Option<(u64, Drew)> {
    if !scene.win.frame.holds(src.shape) {
        return None;
    }
    let got = scene
        .win
        .frame
        .client_room(src.bytes)
        .map(|client| damage_io::read_in(pid, src, plan, client, |_| {}));
    let Some(read) = got? else {
        // Some rows read, not all: no longer a whole picture.
        scene.win.frame.client_shape = None;
        return None;
    };
    Some((read, show_box(scene, a, src.shape.2, b)))
}
