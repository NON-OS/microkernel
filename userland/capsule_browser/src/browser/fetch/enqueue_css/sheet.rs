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

use crate::browser::css::color::media_query_matches;

/// Whether a `<link>` with these attributes (looked up by `attr`) is a
/// stylesheet that applies at a `w` x `h` viewport, as Chromium decides:
/// its rel names `stylesheet` but not `alternate`, it is not `disabled`,
/// its `type` (when given) is text/css, and its `media` list matches.
pub fn sheet_applies<'a>(attr: impl Fn(&str) -> Option<&'a str>, viewport: (u32, u32)) -> bool {
    let rel = attr("rel").unwrap_or("");
    let has = |t: &str| rel.split_ascii_whitespace().any(|r| r.eq_ignore_ascii_case(t));
    if !has("stylesheet") || has("alternate") || attr("disabled").is_some() {
        return false;
    }
    let ty = attr("type").unwrap_or("").split(';').next().unwrap_or("").trim();
    if !ty.is_empty() && !ty.eq_ignore_ascii_case("text/css") {
        return false;
    }
    media_query_matches(attr("media").unwrap_or(""), viewport.0, viewport.1)
}
