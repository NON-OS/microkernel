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

//! Walking a directory document as keyword and arguments, without allocating.

/// One line, split into its keyword and the rest.
pub struct Line<'a> {
    pub keyword: &'a [u8],
    pub rest: &'a [u8],
    /// Where the line starts in the document, which the signature digest needs
    /// in order to name a byte range rather than a line number.
    pub at: usize,
}

/// Iterate the lines of a document.
///
pub fn lines(body: &[u8]) -> impl Iterator<Item = Line<'_>> {
    let mut at = 0usize;
    core::iter::from_fn(move || {
        if at >= body.len() {
            return None;
        }
        let start = at;
        let end = match body[at..].iter().position(|b| *b == b'\n') {
            Some(offset) => at + offset,
            None => body.len(),
        };
        at = core::cmp::min(end + 1, body.len().saturating_add(1));
        let mut text = &body[start..end];
        if text.last() == Some(&b'\r') {
            text = &text[..text.len() - 1];
        }
        let split = text.iter().position(|b| *b == b' ').unwrap_or(text.len());
        Some(Line {
            keyword: &text[..split],
            rest: text.get(split + 1..).unwrap_or(&[]),
            at: start,
        })
    })
}

/// Split arguments on single spaces.
pub fn args(rest: &[u8]) -> impl Iterator<Item = &[u8]> {
    rest.split(|b| *b == b' ').filter(|part| !part.is_empty())
}

/// The nth argument, or `None` when the line is shorter than that.
pub fn arg(rest: &[u8], index: usize) -> Option<&[u8]> {
    args(rest).nth(index)
}
