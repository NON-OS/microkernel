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
    c.font_key = match lower.as_str() {
        "" | "sans-serif" | "serif" | "system-ui" | "ui-sans-serif" => 0,
        "monospace" | "ui-monospace" | "cursive" | "fantasy" => 0,
        _ => crate::browser::fonts::family_key(bare),
    };
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
