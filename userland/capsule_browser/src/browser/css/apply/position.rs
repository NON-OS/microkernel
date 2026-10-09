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

use crate::browser::css::computed::{Computed, Position};
use crate::browser::css::parse_size::parse_offset;
use crate::browser::css::sides::offsets;

/* Positioning scheme and the four inset offsets. fixed lays out like
 * absolute against the viewport and is flagged so paint pins it on scroll.
 * Offsets are signed; `inset` is their 1-4 value shorthand. */
pub(super) fn apply_position(c: &mut Computed, name: &str, value: &str, fs: u32) -> bool {
    let slot = match name {
        "position" => {
            c.is_fixed = false;
            c.is_sticky = false;
            match value.trim() {
                "static" => c.position = Position::Static,
                "relative" => c.position = Position::Relative,
                /* Sticky keeps normal flow, without relative offsets, and
                 * pins at paint time once scrolled past its top threshold. */
                "sticky" => {
                    c.position = Position::Static;
                    c.is_sticky = true;
                }
                "absolute" => c.position = Position::Absolute,
                "fixed" => {
                    c.position = Position::Absolute;
                    c.is_fixed = true;
                }
                _ => {}
            }
            return true;
        }
        "inset" => {
            if let Some([t, r, b, l]) = offsets(value, fs) {
                (c.top, c.right, c.bottom, c.left) = (t, r, b, l);
            }
            return true;
        }
        "top" => &mut c.top,
        "right" => &mut c.right,
        "bottom" => &mut c.bottom,
        "left" => &mut c.left,
        _ => return false,
    };
    if let Some(s) = parse_offset(value, fs) {
        *slot = s;
    }
    true
}
