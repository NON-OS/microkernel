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

use core::f32::consts::PI;

use crate::browser::css::computed::Size;
use crate::browser::css::parse_size::parse_offset;
use crate::browser::layout::boxmodel::Rel;

/// A signed length or percentage as a box-relative (px, per-mille) pair.
/// An undecided min()/max()/clamp() has no single pair and is rejected.
pub(super) fn rel_len(s: &str, fs: u32) -> Option<Rel> {
    match parse_offset(s, fs)? {
        Size::Px(p) => Some((p as i32, 0)),
        Size::Pct(p) => Some((0, p as i32 * 10)),
        Size::Calc(px, pml) => Some((px, pml)),
        Size::Auto | Size::Math(_) => None,
    }
}

/// A plain number, or a percentage as its fraction (scale(50%) is 0.5).
pub(super) fn number(s: &str) -> Option<f32> {
    let s = s.trim();
    let (digits, k) = s.strip_suffix('%').map_or((s, 1.0), |d| (d, 0.01));
    digits.trim().parse::<f32>().ok().filter(|f| f.is_finite()).map(|f| f * k)
}

/// An angle in radians from deg, rad, grad or turn; a bare 0 is zero.
pub(super) fn angle(s: &str) -> Option<f32> {
    let s = s.trim().to_ascii_lowercase();
    let units = [("deg", PI / 180.0), ("grad", PI / 200.0), ("rad", 1.0), ("turn", 2.0 * PI)];
    for (unit, k) in units {
        if let Some(n) = s.strip_suffix(unit) {
            return n.trim().parse::<f32>().ok().filter(|f| f.is_finite()).map(|f| f * k);
        }
    }
    (s == "0").then_some(0.0)
}
