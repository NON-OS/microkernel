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

use alloc::vec::Vec;

use super::scan::find_top;

/* The operands of a top-level and/or chain, and whether it is an 'or'. */
pub(super) fn split_ops(c: &str) -> (Vec<&str>, bool) {
    let (mut parts, mut rest, mut any) = (Vec::new(), c, false);
    loop {
        let at = find_top(rest, b" \t\n");
        let word = rest.get(at..).map(str::trim_start).unwrap_or("");
        let op = word.split(|ch: char| ch.is_whitespace() || ch == '(').next().unwrap_or("");
        if at >= rest.len() || !(op.eq_ignore_ascii_case("and") || op.eq_ignore_ascii_case("or")) {
            parts.push(rest.trim());
            return (parts, any);
        }
        any |= op.eq_ignore_ascii_case("or");
        parts.push(rest[..at].trim());
        rest = &word[op.len()..];
    }
}
