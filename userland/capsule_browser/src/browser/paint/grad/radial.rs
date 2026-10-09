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

use alloc::vec::Vec;

use super::painter::Painter;
use super::split::split_top;
use super::sqrt::isqrt;
use super::stop_list::parse_stops;

/* Parse radial-gradient(...) stops, ignoring the shape and position prelude,
 * which we approximate as a circle centered on the box out to its corner. */
pub(super) fn parse_radial(func: &str) -> Option<Vec<(u32, f32)>> {
    let inner = func.strip_prefix("radial-gradient(")?.strip_suffix(')')?;
    let mut items = split_top(inner);
    if items.is_empty() {
        return None;
    }
    /* A leading prelude (circle, size, "at <pos>") holds no color, so drop it. */
    if items[0].contains("at ")
        || items[0].contains("circle")
        || items[0].contains("ellipse")
        || items[0].contains("closest")
        || items[0].contains("farthest")
    {
        items.remove(0);
    }
    parse_stops(&items)
}

/* Row y of a radial gradient from column x0. The table index is
 * floor(n * d / R) for the doubled distance d to the centre and doubled
 * corner radius R, kept exact in integers: j is found once per row with an
 * integer square root, then nudged as the squared distance changes by
 * 4x + 4 per step, so each pixel costs a few multiplies and compares. */
pub(super) fn radial_row(p: &Painter, size: [i64; 2], r2: u128, at: [i32; 2], out: &mut [u32]) {
    let n2 = (p.n * p.n) as u128;
    let yy = 2 * at[1] as i64 - size[1];
    let mut xx = 2 * at[0] as i64 - size[0];
    let mut d2 = (xx as i128 * xx as i128 + yy as i128 * yy as i128) as u128;
    let mut j = isqrt(n2 * d2 / r2);
    for o in out.iter_mut() {
        let q = n2 * d2;
        while (j + 1) * (j + 1) * r2 <= q {
            j += 1;
        }
        while j * j * r2 > q {
            j -= 1;
        }
        *o = p.lut[(j as usize).min(p.n)];
        d2 = (d2 as i128 + 4 * xx as i128 + 4) as u128;
        xx += 2;
    }
}
