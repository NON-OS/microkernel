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

/// The longest prefix of `css` within `limit` bytes that ends just after a
/// rule's closing brace at nesting depth 0 (0 when none fits).
pub(super) fn rule_end(css: &str, limit: usize) -> usize {
    let (mut depth, mut end) = (0u32, 0);
    for (i, b) in css.bytes().enumerate().take(limit) {
        match b {
            b'{' => depth += 1,
            b'}' => depth = depth.saturating_sub(1),
            _ => continue,
        }
        if b == b'}' && depth == 0 {
            end = i + 1;
        }
    }
    end
}
