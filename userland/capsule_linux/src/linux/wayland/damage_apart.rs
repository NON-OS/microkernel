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

//! The boxes a client said changed apart inside its box (damage_boxes.rs),
//! of a buffer shown at its own size: each read from the guest alone into
//! the picture it lands in, as damage_place.rs reads the one box, and told
//! to the compositor apart (present/draw.rs draw_parts), so the rows
//! between them are neither read out of the guest nor composed again.

use nonos_toolkit::decorations::frame_bg;

use super::super::super::super::present::draw::{draw_parts, Drew};
use super::super::super::super::present::frame::geometry::Around;
use super::super::super::super::present::source::Source;
use super::super::super::super::scene::Scene;
use super::super::damage_boxes::{Boxes, MAX_BOXES};
use super::damage_io::{self, Corners, Reads};
use super::{counted, damage_lay, placed};

/// Each of `parts` read into the window's buffer, flattened onto the
/// frame's ground; None when a read failed, and the window is shown whole.
pub fn unframed(
    pid: u32,
    scene: &mut Scene,
    src: &Source,
    (b, parts): (Corners, &Boxes),
) -> Option<(u64, Drew)> {
    let plans = plans(src, parts)?;
    let (x0, y0, x1, y1) = b;
    let changed = (x0, y0, x1 - x0, y1 - y0);
    let (areas, n) = parts.areas();
    let ground = frame_bg();
    let mut read = None;
    let drew = draw_parts(scene, src.bytes, &areas[..n], |_, out| {
        let flat = |px: &mut [u8]| nonos_pixel_rows::flatten(px, ground);
        let got = read_all(&plans, |r| damage_io::read_in(pid, src, r, out, flat));
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

/// Each of `parts` read into the guest's pixels kept whole for the frame's
/// repaints, then laid into the window and told apart. Only over a picture
/// of this shape, as damage_place.rs framed.
pub fn framed(
    pid: u32,
    scene: &mut Scene,
    src: &Source,
    a: Around,
    (b, parts): (Corners, &Boxes),
) -> Option<(u64, Drew)> {
    if !scene.win.frame.holds(src.shape) {
        return None;
    }
    let plans = plans(src, parts)?;
    let got = scene
        .win
        .frame
        .client_room(src.bytes)
        .map(|client| read_all(&plans, |r| damage_io::read_in(pid, src, r, client, |_| {})));
    let Some(read) = got? else {
        // Some rows read, not all: no longer a whole picture.
        scene.win.frame.client_shape = None;
        return None;
    };
    Some((read, show_parts(scene, a, src.shape.2, (b, parts))))
}

/// The reads of each part; None when one is not wholly on the buffer's
/// bytes, which a part inside a box already planned never is.
fn plans(src: &Source, parts: &Boxes) -> Option<[Option<Reads>; MAX_BOXES]> {
    let mut out = [None; MAX_BOXES];
    for (o, c) in out.iter_mut().zip(parts.iter()) {
        *o = Some(damage_io::plan(src.shape, src.read, c)?);
    }
    Some(out)
}

/// The bytes `read` took over every plan, or None at the first that failed.
fn read_all(
    plans: &[Option<Reads>; MAX_BOXES],
    mut read: impl FnMut(Reads) -> Option<u64>,
) -> Option<u64> {
    plans.iter().flatten().try_fold(0u64, |sum, &r| sum.checked_add(read(r)?))
}

/// The parts of the kept client pixels, `stride` bytes a row, laid into the
/// framed window `a` and presented apart inside the box round them, `b`.
fn show_parts(scene: &mut Scene, a: Around, stride: u32, (b, parts): (Corners, &Boxes)) -> Drew {
    let (_, changed) = placed(a, b);
    let mut laid = [(0, 0, 0, 0); MAX_BOXES];
    let mut cut = [(0, 0, 0, 0); MAX_BOXES];
    for ((l, c), p) in laid.iter_mut().zip(cut.iter_mut()).zip(parts.iter()) {
        (*c, *l) = placed(a, p);
    }
    let n = parts.count();
    let bytes = a.w as usize * a.h as usize * 4;
    let ground = Some(frame_bg());
    draw_parts(scene, bytes, &laid[..n], |scene, out| {
        let client = &scene.win.frame.client;
        let lay = |&r: &Corners| {
            let ok = damage_lay::lay(out, (a.w * 4, a.cx, a.cy), client, stride, r, ground);
            counted(ok, r)
        };
        cut[..n].iter().all(lay).then_some(changed)
    })
}
