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

use crate::browser::css::color::parse_color;
use crate::browser::css::computed::Computed;

use super::font_size::font_size;

/* Text properties: color, weight, family, size, line height and decoration;
 * wrapping, spacing and alignment go on to apply_text_flow. */
pub(super) fn apply_text(
    c: &mut Computed,
    name: &str,
    value: &str,
    fs: u32,
    parent_fs: u32,
) -> bool {
    match name {
        "color" => {
            if let Some(rgb) = parse_color(value) {
                c.color = rgb;
            }
        }
        "font-weight" => super::font_family::apply_font_weight(c, value),
        "font-family" => super::font_family::apply_font_family(c, value),
        "font-style" => super::font_family::apply_font_style(c, value),
        /* em and % resolve against the parent's size, kept unrounded when
         * the style was inherited from that parent (its whole-pixel size
         * agrees with the cascade's); the whole-pixel size is what the box
         * model reads. */
        "font-size" => {
            let exact = (c.em_parent - parent_fs as f32).abs() < 1.0;
            let parent = if exact { c.em_parent } else { parent_fs as f32 };
            if let Some(px) = font_size(value, parent) {
                c.font_px = px;
                c.font_size_px = (px + 0.5) as u32;
            }
        }
        "text-decoration" | "text-decoration-line" => {
            c.underline = value.split_whitespace().any(|t| t == "underline");
        }
        _ => return super::text_flow::apply_text_flow(c, name, value, fs),
    }
    true
}
