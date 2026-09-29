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

use crate::browser::css::computed::{Computed, TextAlign, TextTransform, WhiteSpace};
use crate::browser::css::parse_line_height::parse_line_height;
use crate::browser::css::parse_px::parse_len_f;

use super::font_size::MAX_FONT_PX;

/* Tracking wider than this either way is a layout trick, not text. */
const MAX_SPACING_PX: f32 = 64.0;

/* Text flow properties: line height, letter spacing, wrapping, case and
 * alignment. */
pub(super) fn apply_text_flow(c: &mut Computed, name: &str, value: &str, fs: u32) -> bool {
    match name {
        /* A bare number stays a multiple of each element's own font size
         * down the tree; a length or percentage computes to px here. */
        "line-height" => match value.trim().parse::<f32>() {
            Ok(k) if k.is_finite() && (0.0..=100.0).contains(&k) => {
                (c.line_ratio, c.line_height_px) = (k, 0);
            }
            Ok(_) => {}
            Err(_) => {
                if let Some(px) = parse_line_height(value, fs) {
                    (c.line_ratio, c.line_height_px) = (0.0, px.min(4 * MAX_FONT_PX as u32));
                }
            }
        },
        "letter-spacing" => {
            /* Negative tracking tightens headings, so the length is signed,
             * and kept fractional: -0.04em of 16px is -0.64px, not -1. */
            let v = value.trim();
            if v.eq_ignore_ascii_case("normal") {
                c.letter_spacing = 0.0;
            } else if let Some(px) = parse_len_f(v, fs) {
                c.letter_spacing = px.clamp(-MAX_SPACING_PX, MAX_SPACING_PX);
            }
        }
        "white-space" => match value.trim() {
            "pre" | "pre-wrap" | "pre-line" | "break-spaces" => c.white_space = WhiteSpace::Pre,
            "nowrap" => c.white_space = WhiteSpace::Nowrap,
            "normal" => c.white_space = WhiteSpace::Normal,
            _ => {}
        },
        "text-transform" => match value.trim() {
            "uppercase" => c.text_transform = TextTransform::Upper,
            "lowercase" => c.text_transform = TextTransform::Lower,
            "capitalize" => c.text_transform = TextTransform::Capitalize,
            "none" => c.text_transform = TextTransform::None,
            _ => {}
        },
        "text-align" => match value.trim() {
            "left" | "start" => c.text_align = TextAlign::Left,
            "center" => c.text_align = TextAlign::Center,
            "right" | "end" => c.text_align = TextAlign::Right,
            _ => {}
        },
        _ => return false,
    }
    true
}
