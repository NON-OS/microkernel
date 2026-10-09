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

use ab_glyph::{Font, GlyphId, ScaleFont};
use spin::Mutex;

use super::lookups::kern_lookups;
use super::pair::pair_x;

/* Each face's kern subtables, found once and keyed like the glyph cache
 * by the address and length of the face's data; forgotten with the glyph
 * cache, whose clearing is when page faces are freed. */
const MAX_FACES: usize = 16;

type Lookups = Vec<Vec<usize>>;

static FACES: Mutex<Vec<(usize, usize, Lookups)>> = Mutex::new(Vec::new());

/// The kerning between `a` and `b` in px: the face's GPOS kern lookups
/// when it has them (each lookup's first covering subtable, summed over
/// lookups), else its legacy kern table.
pub(in crate::font::ttf) fn kern_px<F: Font, S: ScaleFont<F>>(
    sf: &S,
    a: GlyphId,
    b: GlyphId,
) -> f32 {
    let d = sf.font().font_data();
    let key = (d.as_ptr() as usize, d.len());
    let mut faces = FACES.lock();
    let i = match faces.iter().position(|f| (f.0, f.1) == key) {
        Some(i) => i,
        None => {
            if faces.len() >= MAX_FACES {
                faces.remove(0);
            }
            faces.push((key.0, key.1, kern_lookups(d)));
            faces.len() - 1
        }
    };
    if faces[i].2.is_empty() {
        drop(faces);
        return sf.kern(a, b);
    }
    let units: i32 = faces[i]
        .2
        .iter()
        .filter_map(|subs| subs.iter().find_map(|&s| pair_x(d, s, a.0, b.0)))
        .map(i32::from)
        .sum();
    units as f32 * sf.h_scale_factor()
}

/// Forget every face's lookups, with the glyph cache.
pub(in crate::font::ttf) fn clear() {
    FACES.lock().clear();
}
