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

/* The length of the longest prefix of `css`, at most `limit` bytes, that
ends just after a '}' closing a top-level block, braces inside comments
and strings not counting; 0 when no rule ends within the limit. */
pub fn cut(css: &str, limit: usize) -> usize {
    let b = css.as_bytes();
    let end = b.len().min(limit);
    let (mut depth, mut last, mut i) = (0usize, 0usize, 0usize);
    while i < end {
        match b[i] {
            b'/' if b.get(i + 1) == Some(&b'*') => {
                let Some(close) = b[i + 2..].windows(2).position(|w| w == b"*/") else { break };
                i += 2 + close + 2;
                continue;
            }
            q @ (b'"' | b'\'') => {
                i += 1;
                while i < end && b[i] != q && b[i] != b'\n' {
                    i += if b[i] == b'\\' { 2 } else { 1 };
                }
            }
            b'\\' => i += 1,
            b'{' => depth += 1,
            b'}' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    last = i + 1;
                }
            }
            _ => {}
        }
        i += 1;
    }
    last
}
