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

use crate::browser::css::computed::Computed;
use crate::browser::fonts;

/* The monospace generic (or an explicit mono family) switches text to
 * the fixed-pitch face. A named first family keys the custom face the
 * text draws in once its @font-face loads; a generic keeps the
 * built-in one. */
pub(super) fn apply_font_family(c: &mut Computed, value: &str) {
    c.mono = value.to_ascii_lowercase().contains("mono");
    let first = value.split(',').next().unwrap_or("").trim();
    let bare = first.trim_matches('"').trim_matches('\'').trim();
    let lower = bare.to_ascii_lowercase();
    c.icon_font = crate::browser::css::icon_font::is_icon_family(&lower);
    let family = match lower.as_str() {
        "" | "sans-serif" | "serif" | "system-ui" | "ui-sans-serif" => 0,
        "monospace" | "ui-monospace" | "cursive" | "fantasy" => 0,
        _ => fonts::family_key(bare),
    };
    c.font_key = fonts::weighted(family, fonts::weight_of(c.font_key));
}

/* font-weight: the number rides in the font key, where a variable face is
 * set to it; `bold` (600 and up) picks a bold cut or thickens a face that
 * has none. bolder and lighter step from the inherited weight. */
pub(super) fn apply_font_weight(c: &mut Computed, value: &str) {
    let now = fonts::weight_of(c.font_key);
    let w = match value.trim().to_ascii_lowercase().as_str() {
        "normal" | "initial" => 400,
        "bold" => 700,
        "bolder" => [(400, 400), (600, 700), (u16::MAX, 900)]
            .iter()
            .find(|s| now < s.0)
            .map_or(900, |s| s.1),
        "lighter" => [(600, 100), (800, 400), (u16::MAX, 700)]
            .iter()
            .find(|s| now < s.0)
            .map_or(700, |s| s.1),
        v => match v.parse::<f32>().ok().filter(|w| (1.0..=1000.0).contains(w)) {
            Some(w) => w as u16,
            None => return,
        },
    };
    c.bold = w >= 600;
    c.font_key = fonts::weighted(c.font_key, w);
}

/* font-style: italic and oblique (with or without an angle) both draw the
 * face slanted, since no italic cut is loaded; normal draws it upright. */
pub(super) fn apply_font_style(c: &mut Computed, value: &str) {
    let v = value.trim().to_ascii_lowercase();
    match v.split_whitespace().next() {
        Some("italic" | "oblique") => c.italic = true,
        Some("normal") => c.italic = false,
        _ => {}
    }
}
