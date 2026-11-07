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

//! The window a surface is: xdg_surface.set_window_geometry's box, the rest
//! of the buffer being what the client draws outside its window (a GTK
//! shadow: Firefox draws 852x652 for an 800x600 window at 26,23). The
//! window shows that box alone, and what is placed on the surface, the
//! pointer, subsurfaces and popups, is reckoned from its corner. Pure, so
//! the host proofs hold it.

/// A box in buffer pixels: x, y, width, height.
pub type Area = (u32, u32, u32, u32);

/// Geometry `g`, in the surface's logical pixels at buffer scale `k`, as the
/// box of a `w` by `h` buffer the window is; None when that is the whole
/// buffer, or when it leaves nothing of it, which is shown whole.
pub fn crop(g: Option<(i32, i32, i32, i32)>, k: u32, (w, h): (u32, u32)) -> Option<Area> {
    let (gx, gy, gw, gh) = g?;
    let k = i64::from(k.max(1));
    let px = |v: i64| (v * k).clamp(0, i64::from(u32::MAX)) as u32;
    let (x0, y0) = (px(i64::from(gx)), px(i64::from(gy)));
    let x1 = px(i64::from(gx) + i64::from(gw)).min(w);
    let y1 = px(i64::from(gy) + i64::from(gh)).min(h);
    if x0 >= x1 || y0 >= y1 {
        return None;
    }
    let b = (x0, y0, x1 - x0, y1 - y0);
    (b != (0, 0, w, h)).then_some(b)
}

/// Geometry `g` on a surface with a viewport whose destination is `dest`
/// logical pixels, as the box of the `w` by `h` buffer drawn for it, and
/// that box's logical size: the destination is the surface's size, so the
/// geometry is in its pixels, and each axis runs `w` (or `h`) buffer pixels
/// to `dest`'s. Qt 6 gives every surface such a viewport once it is told a
/// fractional scale, 1 included, and Telegram's shadow round its window was
/// shown, flattened, as an opaque band. None as `crop` answers it.
pub fn crop_dest(
    g: Option<(i32, i32, i32, i32)>,
    dest: (u32, u32),
    (w, h): (u32, u32),
) -> Option<(Area, (u32, u32))> {
    let (gx, gy, gw, gh) = g?;
    let (x0, x1) = logical_span(gx, gw, dest.0)?;
    let (y0, y1) = logical_span(gy, gh, dest.1)?;
    let (bx0, bx1) = (to_buffer(x0, w, dest.0), to_buffer(x1, w, dest.0));
    let (by0, by1) = (to_buffer(y0, h, dest.1), to_buffer(y1, h, dest.1));
    if bx0 >= bx1 || by0 >= by1 {
        return None;
    }
    let b = (bx0, by0, bx1 - bx0, by1 - by0);
    (b != (0, 0, w, h)).then_some((b, (x1 - x0, y1 - y0)))
}

/// The part of `at`..`at + len` inside 0..`size`, None when nothing is.
fn logical_span(at: i32, len: i32, size: u32) -> Option<(u32, u32)> {
    let end = (i64::from(at) + i64::from(len)).min(i64::from(size));
    let start = i64::from(at).max(0);
    (start < end).then_some((start as u32, end as u32))
}

/// Logical `v` of `logical` as a pixel of a buffer `buffer` wide, to the
/// nearest, never past the buffer's edge.
fn to_buffer(v: u32, buffer: u32, logical: u32) -> u32 {
    let px =
        (u64::from(v) * u64::from(buffer) + u64::from(logical) / 2) / u64::from(logical.max(1));
    px.min(u64::from(buffer)) as u32
}

/// Where the window starts on its surface, in its logical pixels: geometry
/// `g`'s corner, never left of or above the surface's own, as `crop` takes
/// it. A popup is placed by that corner (xdg_positioner places the window
/// geometry), so its surface starts this far up and left of its place.
pub fn corner(g: Option<(i32, i32, i32, i32)>) -> (i32, i32) {
    g.map_or((0, 0), |(x, y, _, _)| (x.max(0), y.max(0)))
}

/// Where the box `b` of a buffer at `stride` bytes a row starts, and how
/// many bytes from there hold it: its last row ends at its own right edge,
/// so the read never passes the buffer's end.
pub fn span((x, y, w, h): Area, stride: u32) -> (u64, usize) {
    let start = u64::from(y) * u64::from(stride) + u64::from(x) * 4;
    let rows = (h.saturating_sub(1) as usize) * stride as usize;
    (start, rows + w as usize * 4)
}
