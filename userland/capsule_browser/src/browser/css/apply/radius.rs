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

use crate::browser::css::calc::split_top::words;
use crate::browser::css::computed::{Computed, Size};
use crate::browser::css::parse_size::parse_size;

/* Corners are stored top-left, top-right, bottom-right, bottom-left. Each
 * corner is drawn circular: with an elliptical `a / b` value the horizontal
 * radius is kept. Percentages resolve against the box at layout, so 50%
 * rounds a square into a circle and a wide box into a pill. */
pub(super) fn apply_radius(c: &mut Computed, name: &str, value: &str, fs: u32) -> bool {
    let corner = match name {
        "border-radius" => {
            if let Some(r) = shorthand(value, fs) {
                c.radius = r;
            }
            return true;
        }
        "border-top-left-radius" => 0,
        "border-top-right-radius" => 1,
        "border-bottom-right-radius" => 2,
        "border-bottom-left-radius" => 3,
        _ => return false,
    };
    if let Some(r) = words(value).next().and_then(|w| parse_size(w, fs)) {
        c.radius[corner] = r;
    }
    true
}

/* The 1-4 value shorthand, expanded like CSS: one value for all corners;
 * two for top-left/bottom-right then top-right/bottom-left; three with the
 * third for bottom-right; four in clockwise order. */
fn shorthand(value: &str, fs: u32) -> Option<[Size; 4]> {
    let horizontal = value.split('/').next()?;
    let mut v = [Size::Px(0); 4];
    let mut n = 0;
    for w in words(horizontal) {
        *v.get_mut(n)? = parse_size(w, fs)?;
        n += 1;
    }
    match n {
        1 => Some([v[0]; 4]),
        2 => Some([v[0], v[1], v[0], v[1]]),
        3 => Some([v[0], v[1], v[2], v[1]]),
        4 => Some(v),
        _ => None,
    }
}
