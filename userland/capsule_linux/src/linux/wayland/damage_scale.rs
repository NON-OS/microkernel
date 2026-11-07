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

//! The part a client said changed, of a buffer shown at another size, at
//! the size it is shown. Pure, so the host proofs hold it.

/// The shown box of a buffer `from` pixels big shown at `to` whose pixels
/// nearest-neighbour sampling (present/resample.rs) takes from the buffer
/// box `b`: each shown pixel takes the buffer pixel under its centre.
pub fn shown_box(
    b: (u32, u32, u32, u32),
    from: (u32, u32),
    to: (u32, u32),
) -> (u32, u32, u32, u32) {
    /* The first shown d whose centre falls at or past buffer pixel v. */
    let first = |v: u32, f: u32, t: u32| {
        let need = (2 * u64::from(v) * u64::from(t)).saturating_sub(u64::from(f));
        need.div_ceil(2 * u64::from(f.max(1))).min(u64::from(t)) as u32
    };
    let (f, t) = (from, to);
    (first(b.0, f.0, t.0), first(b.1, f.1, t.1), first(b.2, f.0, t.0), first(b.3, f.1, t.1))
}

/// Rows `s.1` up to `s.3` of the shown pixels, `to.0` a row, with the box
/// `s` filled from `rows`, buffer rows from `y0` at `stride` bytes each;
/// None when the heap has no room for them.
pub fn resample_box(
    rows: &[u8],
    (stride, y0): (u32, u32),
    from: (u32, u32),
    to: (u32, u32),
    s: (u32, u32, u32, u32),
) -> Option<alloc::vec::Vec<u8>> {
    let at = |d: u32, f: u32, t: u32| {
        ((2 * u64::from(d) + 1) * u64::from(f) / (2 * u64::from(t.max(1))))
            .min(u64::from(f.max(1) - 1)) as usize
    };
    let row = to.0 as usize * 4;
    let mut out = alloc::vec::Vec::new();
    out.try_reserve_exact(s.3.saturating_sub(s.1) as usize * row).ok()?;
    out.resize(s.3.saturating_sub(s.1) as usize * row, 0);
    // The source column of each shown column, once for the box; a buffer
    // of no width reads its first column, as the rule above gives.
    let Some(cols) = nonos_pixel_rows::NearestCols::new(from.0.max(1), to.0) else {
        return Some(out);
    };
    for (i, dy) in (s.1..s.3).enumerate() {
        let Some(sy) = at(dy, from.1, to.1).checked_sub(y0 as usize) else { continue };
        let Some(line) = rows.get(sy * stride as usize..) else { continue };
        let span = i * row + s.0 as usize * 4..i * row + s.2 as usize * 4;
        let Some(dst) = out.get_mut(span) else { continue };
        nonos_pixel_rows::nearest_row(dst, line, &cols, s.0 as usize);
    }
    Some(out)
}
