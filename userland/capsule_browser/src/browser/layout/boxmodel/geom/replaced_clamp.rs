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

use crate::browser::css::{Computed, Size};

/// Bring an image's tentative size (w, h) within min/max-width and
/// min/max-height. `set` says which sides CSS or an attribute fixed; a
/// side derived from the other follows it by `ratio` (w / h), and with
/// both derived CSS 2.1 10.4's table keeps the ratio. An auto width also
/// stays within `avail`, the line; `cb_h` resolves percentage heights.
pub(crate) fn clamp_replaced(
    s: &Computed,
    (w, h): (i32, i32),
    set: (bool, bool),
    ratio: Option<f32>,
    (avail, cb_h): (i32, Option<i32>),
) -> (i32, i32) {
    let mut wb = bounds(s.min_width.resolve(avail), s.max_width.resolve(avail));
    if !set.0 {
        wb.1 = wb.1.min(avail.max(0) as f32).max(wb.0);
    }
    let hr = |z: Size| z.definite_px().or_else(|| cb_h.and_then(|b| z.resolve(b)));
    let hb = bounds(hr(s.min_height), hr(s.max_height));
    let (w, h) = (w as f32, h as f32);
    let c = |v: f32, b: (f32, f32)| v.clamp(b.0, b.1);
    let (w, h) = match (set, ratio) {
        ((false, false), Some(_)) if w > 0. && h > 0. => table((w, h), wb, hb),
        ((true, false), Some(r)) => (c(w, wb), c(c(w, wb) / r, hb)),
        ((false, true), Some(r)) => (c(c(h, hb) * r, wb), c(h, hb)),
        _ => (c(w, wb), c(h, hb)),
    };
    ((w + 0.5) as i32, (h + 0.5) as i32)
}

/* A side's (min, max) in px, max never below min. */
fn bounds(min: Option<i32>, max: Option<i32>) -> (f32, f32) {
    let lo = min.unwrap_or(0).max(0) as f32;
    (lo, max.map_or(f32::INFINITY, |m| m as f32).max(lo))
}

/* The CSS 2.1 10.4 table for a replaced box with a ratio and both sides
 * auto: the violated bounds are met while the ratio holds where it can. */
fn table((w, h): (f32, f32), (w0, w1): (f32, f32), (h0, h1): (f32, f32)) -> (f32, f32) {
    match (w > w1, h > h1, w < w0, h < h0) {
        (true, true, _, _) if w1 / w <= h1 / h => (w1, (w1 * h / w).max(h0)),
        (true, true, _, _) => ((h1 * w / h).max(w0), h1),
        (_, _, true, true) if w0 / w <= h0 / h => ((h0 * w / h).min(w1), h0),
        (_, _, true, true) => (w0, (w0 * h / w).min(h1)),
        (_, true, true, _) => (w0, h1),
        (true, _, _, true) => (w1, h0),
        (true, ..) => (w1, (w1 * h / w).max(h0)),
        (_, _, true, _) => (w0, (w0 * h / w).min(h1)),
        (_, true, ..) => ((h1 * w / h).max(w0), h1),
        (.., true) => ((h0 * w / h).min(w1), h0),
        _ => (w, h),
    }
}
