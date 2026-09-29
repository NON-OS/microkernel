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

use ab_glyph::{Font, FontRef, GlyphId, PxScale, ScaleFont};

use super::cache;
use super::glyph::{draw_glyph, face_id, Place};
use super::store::Key;
use super::target::Target;

/* The phase that marks a chrome raster: drawn at the pen origin, unlike the
page path's quarter-pixel phases 0..=3. */
const ORIGIN: u8 = u8::MAX;

/* The hot path for body, gutter and labels: each glyph is rasterized once at
the pen origin and blitted at the truncated pen and baseline, so a full
screen of crisp text stays cheap enough to repaint on every keystroke. */
pub(super) fn draw_cached(
    f: &FontRef,
    t: &mut Target,
    at: (i32, i32),
    text: &str,
    argb: u32,
    px: f32,
) -> i32 {
    let sf = f.as_scaled(PxScale::from(px));
    let baseline = (at.1 as f32 + sf.ascent()) as i32;
    let (face, len) = face_id(f);
    let mut pen = at.0 as f32;
    let mut prev: Option<GlyphId> = None;
    let mut store = cache::lock();
    for ch in text.chars() {
        let g = sf.scaled_glyph(ch);
        if let Some(p) = prev {
            pen += sf.kern(p, g.id);
        }
        let adv = sf.h_advance(g.id);
        prev = Some(g.id);
        let key = Key { face, len, glyph: g.id.0, px: px.to_bits(), phase: ORIGIN };
        let place = Place { x: pen as i32, y: baseline, argb, bias: 0.0 };
        draw_glyph(&mut store, t, key, || sf.outline_glyph(g), place);
        pen += adv;
    }
    pen as i32
}
