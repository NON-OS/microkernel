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

use alloc::string::String;
use alloc::vec::Vec;

use spin::Mutex;

/// A CSS filter list reduced to one affine color map: rows r, g, b of
/// [r, g, b, offset] in 16.16 fixed point (offset in 0..255 units), then
/// the alpha scale. Only the color functions compose this way.
pub type Tint = [i32; 13];

/* Filter values seen so far with their maps, each kept once; a box holds
 * its index plus one. Past MAX_TINTS distinct values a new one gets 0 and
 * paints unfiltered. */
const MAX_TINTS: usize = 256;

static TINTS: Mutex<Vec<(String, Tint)>> = Mutex::new(Vec::new());

/// The id of filter value `v`, its map made by `parse` on first sight: 0
/// when it holds no color function this renderer applies (none, a lone
/// blur or drop-shadow) or the table is full.
pub fn tint_id(v: &str, parse: fn(&str) -> Option<Tint>) -> u16 {
    let v = v.trim();
    let mut t = TINTS.lock();
    if let Some(i) = t.iter().position(|(s, _)| s == v) {
        return i as u16 + 1;
    }
    let Some(m) = parse(v) else { return 0 };
    if t.len() >= MAX_TINTS {
        return 0;
    }
    t.push((String::from(v), m));
    t.len() as u16
}

/// The color map behind `id`.
pub fn tint_of(id: u16) -> Option<Tint> {
    TINTS.lock().get((id as usize).checked_sub(1)?).map(|e| e.1)
}

/// One ARGB pixel through the map, each channel clamped to 0..=255.
pub fn tint_px(m: &Tint, px: u32) -> u32 {
    let (r, g, b) = ((px >> 16 & 0xff) as i64, (px >> 8 & 0xff) as i64, (px & 0xff) as i64);
    let row = |k: usize| {
        let v = m[k] as i64 * r + m[k + 1] as i64 * g + m[k + 2] as i64 * b;
        ((v + m[k + 3] as i64 * 255 + 0x8000) >> 16).clamp(0, 255) as u32
    };
    let a = (((px >> 24) as i64 * m[12] as i64 + 0x8000) >> 16).clamp(0, 255) as u32;
    (a << 24) | (row(0) << 16) | (row(4) << 8) | row(8)
}
