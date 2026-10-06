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

use super::super::key::family_key;
use super::super::pick_src::{pick_src, split_top};
use super::descriptor::{covers_latin, weight_range};
use alloc::string::String;

/// One @font-face: family key, weight range, whether it is italic or
/// oblique, whether its unicode-range reaches Latin, and its source.
pub(super) struct Face {
    pub(super) key: u32,
    pub(super) weight: (u16, u16),
    pub(super) italic: bool,
    pub(super) latin: bool,
    pub(super) url: String,
}

/// The face one @font-face rule body declares, if it names a family and
/// a source the engine loads. A font-weight that does not parse leaves
/// the face at normal, as an invalid descriptor is dropped.
pub(super) fn face(body: &str) -> Option<Face> {
    let key = family_key(decl_value(body, "font-family")?);
    let url = pick_src(decl_value(body, "src")?)?;
    if key == 0 {
        return None;
    }
    let weight = decl_value(body, "font-weight").and_then(weight_range).unwrap_or((400, 400));
    let style = decl_value(body, "font-style").unwrap_or("normal").to_ascii_lowercase();
    let italic = style.starts_with("italic") || style.starts_with("oblique");
    let latin = covers_latin(decl_value(body, "unicode-range"));
    Some(Face { key, weight, italic, latin, url })
}

/* The value of the first `name:` declaration in a rule body. Splitting on
 * top-level semicolons and then the first colon keeps urls whole. */
fn decl_value<'a>(body: &'a str, name: &str) -> Option<&'a str> {
    for decl in split_top(body, b';') {
        let Some((n, v)) = decl.split_once(':') else { continue };
        if n.trim().eq_ignore_ascii_case(name) {
            return Some(v.trim());
        }
    }
    None
}
