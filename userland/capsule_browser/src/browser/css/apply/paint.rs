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

use crate::browser::css::bg_url;
use crate::browser::css::color::parse_color;
use crate::browser::css::computed::Computed;
use crate::browser::layout::filter_table;

/* Painted appearance: background color, size and repeat and the filter
 * color map here; corner radii
 * in apply_radius; stacking, overflow, opacity and effects in apply_visual. */
pub(super) fn apply_paint(c: &mut Computed, name: &str, value: &str, fs: u32) -> bool {
    match name {
        "background-color" => {
            if value.trim().eq_ignore_ascii_case("currentcolor") {
                c.bg = c.color;
            } else if let Some(rgb) = parse_color(value) {
                c.bg = rgb;
            }
        }
        "background" => bg_url::apply_background(c, value, fs),
        "background-size" | "mask-size" | "-webkit-mask-size" => {
            bg_url::apply_bg_size(c, value, fs)
        }
        "filter" | "-webkit-filter" => {
            c.fx.tint = filter_table::tint_id(value, super::filter_parse::parse_filter)
        }
        "background-position" | "mask-position" | "-webkit-mask-position" => {
            bg_url::apply_bg_pos(c, value, fs)
        }
        "background-repeat" | "mask-repeat" | "-webkit-mask-repeat" => {
            c.bg_layer.repeat = !value.split(',').next().unwrap_or("").contains("no-repeat");
        }
        _ if name.ends_with("radius") => return super::radius::apply_radius(c, name, value, fs),
        _ => return super::visual::apply_visual(c, name, value, fs),
    }
    true
}
