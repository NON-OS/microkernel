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

//! Choosing the largest of a set of text sizes at which a run fits a width.

use crate::paint::measure_ttf;

/// The largest size in `sizes` at which `s` measures within `max_w`, or `None`
/// when it fits at none of them. Every size is measured, in any order the
/// caller lists them, since a run need not narrow with each smaller size once
/// sizes are clamped to a readable floor. A size listed twice is one candidate,
/// and a NaN size is never chosen.
pub fn fit_size_by(
    s: &str,
    max_w: i32,
    sizes: &[f32],
    measure: impl Fn(&str, f32) -> i32,
) -> Option<f32> {
    let fits = sizes.iter().copied().filter(|px| !px.is_nan() && measure(s, *px) <= max_w);
    fits.reduce(f32::max)
}

/// `fit_size_by` measured in the body face, the face `draw::text` paints with.
pub fn fit_size(s: &str, max_w: u32, sizes: &[f32]) -> Option<f32> {
    fit_size_by(s, i32::try_from(max_w).unwrap_or(i32::MAX), sizes, measure_ttf)
}
