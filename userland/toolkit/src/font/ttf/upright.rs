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

use ab_glyph::{point, Font, GlyphId, PxScale, ScaleFont};

use super::cache;
use super::glyph::{draw_glyph, face_id, Place};
use super::raster::floor;
use super::store::Key;
use super::target::Target;

/* Subpixel x positions a glyph is rasterised at: quarter pixels. */
const PHASES: f32 = 4.0;

/*
 * Page text, upright, through the glyph cache. The pen advances in floats
 * exactly as before, so the returned x still matches `measure_tracked`; each
 * glyph is drawn at its pen rounded to the nearest quarter pixel (a whole
 * pixel cell plus a phase the coverage is rasterised at) and at the
 * baseline's own fraction, which is the ascent's fraction because every line
 * top is a whole pixel. No glyph lands more than an eighth of a pixel from
 * where exact placement would put it.
 */
pub(super) fn draw_upright<F: Font>(
    f: &F,
    t: &mut Target,
    at: (i32, i32),
    text: &str,
    argb: u32,
    px: f32,
    spacing: f32,
) -> i32 {
    let sf = f.as_scaled(PxScale::from(px));
    let ascent = sf.ascent();
    let lift = floor(ascent);
    let (baseline, frac) = (at.1.saturating_add(lift as i32), ascent - lift);
    let (face, len) = face_id(f);
    let mut pen = at.0 as f32;
    let mut prev: Option<GlyphId> = None;
    let mut store = cache::lock();
    for ch in text.chars() {
        let id = sf.glyph_id(ch);
        if let Some(p) = prev {
            pen += sf.kern(p, id);
        }
        let adv = sf.h_advance(id);
        prev = Some(id);
        let quarters = floor(pen * PHASES + 0.5);
        let cell = floor(quarters / PHASES);
        let phase = ((quarters - cell * PHASES) as u8).min(PHASES as u8 - 1);
        let key = Key { face, len, glyph: id.0, px: px.to_bits(), phase };
        let mut g = id.with_scale(sf.scale());
        g.position = point(phase as f32 / PHASES, frac);
        let place = Place { x: cell as i32, y: baseline, argb, bias: 0.5 };
        draw_glyph(&mut store, t, key, || sf.outline_glyph(g), place);
        pen += adv + spacing;
    }
    pen as i32
}
