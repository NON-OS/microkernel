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

/* The ceiling on any parsed length. Without it a value such as
 * width:4000000000px reaches layout as ~4.29e9 and overflows the i32
 * box-model arithmetic, which aborts under release overflow-checks. */
pub(super) const MAX_LEN_PX: f32 = 100_000.0;

/// A signed length in px: a number with any length unit, or a math function
/// (calc, min, max, clamp) whose result has no percentage part. Zero may be
/// written without a unit. Anything else is rejected.
pub(super) fn parse_len_f(value: &str, em_base: u32) -> Option<f32> {
    match eval_value(value, em_base as f32)? {
        V::Num(n) if n == 0.0 => Some(0.0),
        V::Len { px, pml } if pml == 0.0 && px.is_finite() && px.abs() <= MAX_LEN_PX => Some(px),
        _ => None,
    }
}

/// Resolve a CSS length to whole non-negative pixels; em resolves against
/// `em_base`, rem against the root element's font size, viewport units against the
/// viewport the cascade published. A negative length is rejected.
pub(super) fn parse_px(value: &str, em_base: u32) -> Option<u32> {
    let px = parse_len_f(value, em_base)?;
    (px >= 0.0).then(|| (px + 0.5) as u32)
}

/// A signed margin: px, and per-mille of the containing block's width for
/// a percentage part, each rounded half away from zero. A min()/max()
/// comparison involving a percentage has no single such pair and drops.
pub(super) fn parse_margin(value: &str, em_base: u32) -> Option<(i32, i32)> {
    let (px, pml) = match eval_value(value, em_base as f32)? {
        V::Num(n) if n == 0.0 => (0.0, 0.0),
        V::Len { px, pml } => (px, pml),
        _ => return None,
    };
    let ok = |v: f32| v.is_finite() && v.abs() <= MAX_LEN_PX;
    let round = |v: f32| if v < 0.0 { (v - 0.5) as i32 } else { (v + 0.5) as i32 };
    (ok(px) && ok(pml)).then(|| (round(px), round(pml)))
}
