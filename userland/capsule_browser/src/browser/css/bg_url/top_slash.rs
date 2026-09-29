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

/// Byte offset of the position/size slash: the first `/` outside any
/// parenthesis or quote, so a slash in url(/a/b.png) is not it.
pub(super) fn top_slash(layer: &str) -> Option<usize> {
    let (mut depth, mut quote) = (0i32, 0u8);
    for (i, b) in layer.bytes().enumerate() {
        match b {
            _ if quote != 0 => quote = if b == quote { 0 } else { quote },
            b'"' | b'\'' => quote = b,
            b'(' => depth += 1,
            b')' => depth -= 1,
            b'/' if depth <= 0 => return Some(i),
            _ => {}
        }
    }
    None
}
