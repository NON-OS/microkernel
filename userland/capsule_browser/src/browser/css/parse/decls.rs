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

use alloc::string::ToString;
use alloc::vec::Vec;

use crate::browser::css::decl::Decl;

use super::parse_into::scan::split_top;

/* Declarations kept from one block; a :root palette of a few thousand
 * custom properties fits, and the sheet's byte cap bounds the rest. */
const MAX_DECLS: usize = 8192;

/* The declarations of a block or a style attribute. Items split at
 * semicolons outside strings and brackets, so url(data:...;base64,...) and
 * content: ";" stay whole. */
pub fn parse_decls(block: &str) -> Vec<Decl> {
    let mut out: Vec<Decl> = Vec::new();
    for item in split_top(block, b';') {
        if out.len() >= MAX_DECLS {
            break;
        }
        if let Some(d) = parse_decl(item) {
            out.push(d);
        }
    }
    out
}

/* One "name: value" item, with a trailing !important taken off the value
 * and kept as the declaration's importance. A custom property may be
 * empty; any other property needs a value. */
pub(super) fn parse_decl(item: &str) -> Option<Decl> {
    let colon = item.find(':')?;
    let name = item[..colon].trim();
    if name.is_empty() || name.contains(char::is_whitespace) {
        return None;
    }
    let (value, important) = split_important(item[colon + 1..].trim());
    let custom = name.starts_with("--");
    if value.is_empty() && !custom {
        return None;
    }
    Some(Decl::new(name.to_ascii_lowercase(), value.to_string(), important))
}

/* "red ! IMPORTANT" is ("red", true); anything else keeps its text. */
fn split_important(v: &str) -> (&str, bool) {
    let n = v.len();
    if n < 9 || !v.is_char_boundary(n - 9) || !v[n - 9..].eq_ignore_ascii_case("important") {
        return (v, false);
    }
    match v[..n - 9].trim_end().strip_suffix('!') {
        Some(rest) => (rest.trim_end(), true),
        None => (v, false),
    }
}
