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

//! Zips the emblem against the info rows into one column pair.

use alloc::vec::Vec;

use super::emblem_shape::EMBLEM_W;

/// A row twice: as written to a file, and as the screen shows it.
pub type Row = (Vec<u8>, Vec<u8>);

/// Lay the emblem's rows `left` beside `right`, the left column padded to
/// `EMBLEM_W + gap` cells, running to whichever side is longer.
///
/// The padding counts characters, not bytes: the emblem's blocks are three
/// bytes each, so a byte-based pad would push the info column off to the
/// right. Trailing blanks are dropped from the plain row.
pub fn two_column(left: &[Row], right: &[Row], gap: usize) -> Vec<Row> {
    let width = EMBLEM_W + gap;
    let rows = core::cmp::max(left.len(), right.len());
    let mut out = Vec::with_capacity(rows);
    for index in 0..rows {
        let (mut plain, mut styled) = left.get(index).cloned().unwrap_or_default();
        let cells = core::str::from_utf8(&plain).map_or(plain.len(), |p| p.chars().count());
        let pad = width.saturating_sub(cells);
        plain.resize(plain.len() + pad, b' ');
        styled.resize(styled.len() + pad, b' ');
        if let Some((p, s)) = right.get(index) {
            plain.extend_from_slice(p);
            styled.extend_from_slice(s);
        }
        while plain.last() == Some(&b' ') {
            plain.pop();
        }
        out.push((plain, styled));
    }
    out
}
