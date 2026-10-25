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

//! Kernel source read as code alone, for the glue checks that pin it whole.

/// The code alone: comments cut, whitespace dropped.
pub(super) fn code(text: &str) -> String {
    let cut = |l: &str| l.split("//").next().unwrap_or("").split_whitespace().collect::<String>();
    text.lines().map(cut).collect()
}

/// The code of `text` from `from` up to `to`.
pub(super) fn between(text: &str, from: &str, to: &str) -> String {
    let (c, from, to) = (code(text), code(from), code(to));
    let a = c.find(&from).unwrap_or_else(|| panic!("missing `{from}`"));
    let b = c[a..].find(&to).unwrap_or_else(|| panic!("missing `{to}`"));
    c[a..a + b].to_string()
}
