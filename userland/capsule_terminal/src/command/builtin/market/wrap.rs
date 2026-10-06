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

//! A description, cut into lines no wider than the screen, at spaces where
//! there is one.

use alloc::vec::Vec;

pub(super) fn wrap(text: &[u8], width: usize) -> Vec<&[u8]> {
    let width = width.max(1);
    let mut lines = Vec::new();
    let mut rest = text;
    while !rest.is_empty() {
        if rest.len() <= width {
            lines.push(rest);
            break;
        }
        let cut = rest[..=width].iter().rposition(|b| *b == b' ').filter(|&at| at > 0);
        let (line, next) = match cut {
            Some(at) => (&rest[..at], &rest[at + 1..]),
            None => (&rest[..width], &rest[width..]),
        };
        lines.push(line);
        rest = next;
    }
    lines
}
