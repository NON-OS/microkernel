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

use super::calc::{eval_value, V};
use super::computed::Size;
use super::parse_px::MAX_LEN_PX;

/* Largest percentage kept; ten times the base is past any real layout. */
const MAX_PML: f32 = 10_000.0;

/// Resolve a width/height value: auto, a length, a percentage of the
/// containing box, a calc() carried to layout as px plus per-mille, or a
/// min()/max()/clamp() whose percentage arguments layout decides. A bare
/// negative length is invalid for a size.
pub(super) fn parse_size(value: &str, em_base: u32) -> Option<Size> {
    if value.trim_start().starts_with('-') {
        return None;
    }
    parse_offset(value, em_base)
}

/// The same forms with negative values allowed, for top/right/bottom/left.
pub(super) fn parse_offset(value: &str, em_base: u32) -> Option<Size> {
    let v = value.trim();
    if v.eq_ignore_ascii_case("auto") {
        return Some(Size::Auto);
    }
    let (px, pml) = match eval_value(v, em_base as f32)? {
        V::Num(n) if n == 0.0 => return Some(Size::Px(0)),
        V::Num(_) => return None,
        V::Math(m) => return Some(Size::Math(m)),
        V::Len { px, pml } => (px, pml),
    };
    if !px.is_finite() || !pml.is_finite() || px.abs() > MAX_LEN_PX || pml.abs() > MAX_PML {
        return None;
    }
    let round = |f: f32| if f < 0.0 { (f - 0.5) as i32 } else { (f + 0.5) as i32 };
    Some(match (round(px), round(pml)) {
        /* A whole percentage literal keeps the plain form; 12.5% stays exact
         * as per-mille. */
        (0, pml) if pml >= 0 && pml % 10 == 0 && v.ends_with('%') => Size::Pct((pml / 10) as u16),
        (px, 0) if px >= 0 => Size::Px(px as u32),
        (px, pml) => Size::Calc(px, pml),
    })
}
