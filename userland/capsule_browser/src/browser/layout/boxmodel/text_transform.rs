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

use alloc::string::{String, ToString};

use crate::browser::css::TextTransform;

/* The text a source word paints, so the glyphs layout measures are the
 * ones drawn: text-transform applied (capitalize only where a word starts,
 * not where an element boundary splits one), or for an icon-font word, the
 * symbol its ligature name stands for, else nothing, since the icon face
 * itself is not loaded and the name must not show as a literal word. */
pub(super) fn transform(w: &str, mode: TextTransform, word_start: bool, icon: bool) -> String {
    if icon {
        return String::from(crate::browser::css::icon_font::map_ligature(w).unwrap_or(""));
    }
    match mode {
        TextTransform::None => w.to_string(),
        TextTransform::Upper => w.to_uppercase(),
        TextTransform::Lower => w.to_lowercase(),
        TextTransform::Capitalize if word_start => {
            let mut chars = w.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect(),
                None => String::new(),
            }
        }
        TextTransform::Capitalize => w.to_string(),
    }
}
