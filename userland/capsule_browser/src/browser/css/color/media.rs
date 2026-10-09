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

use alloc::format;

/// Whether the media query list `query` (a link or style `media`
/// attribute, or an @import's list) holds at a `w` x `h` viewport. It is
/// judged by the stylesheet parser itself, so an attribute and an @media
/// block with the same text always agree. An empty list matches; one that
/// could not be an @media prelude (braces or a semicolon) does not.
///
/// This is a stopgap home: the parser's own matcher is private to it.
pub fn media_query_matches(query: &str, w: u32, h: u32) -> bool {
    let q = query.trim();
    if q.is_empty() {
        return true;
    }
    if q.contains(['{', '}', ';']) {
        return false;
    }
    let _held = crate::browser::css::calc::viewport::enter(w, h);
    let sheet = format!("@media {q}{{a{{color:red}}}}");
    /* The parse records a verdict rule for every condition; the query holds
     * when the style rule inside it survived. */
    let rules = crate::browser::css::parse::parse(&sheet);
    rules.iter().any(|r| r.flags & crate::browser::css::rule::Rule::COND == 0)
}
