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

//! Breaking a run into lines that each fit a width, cutting inside a word
//! when the word alone is wider than a line.

use alloc::vec::Vec;

use super::clip::clip_by;
use super::wrap::words;
use crate::paint::measure_ttf;

/// Byte ranges of `text` broken into lines no wider than `max_w`, as
/// `wrap_by` breaks them, except that a word wider than `max_w` starts a line
/// of its own and is cut, at the last char boundary that fits, into as many
/// lines as it needs; its last piece may share a line with the words after
/// it. A line is one char over `max_w` only when that char alone is wider.
pub fn wrap_hard_by(text: &str, max_w: i32, measure: impl Fn(&str) -> i32) -> Vec<(usize, usize)> {
    let mut lines = Vec::new();
    let mut line: Option<(usize, usize)> = None;
    for (start, end) in words(text) {
        if let Some((s, _)) = line.filter(|&(s, _)| measure(&text[s..end]) <= max_w) {
            line = Some((s, end));
            continue;
        }
        lines.extend(line);
        let mut from = start;
        while from < end && measure(&text[from..end]) > max_w {
            let cut = from + piece(&text[from..end], max_w, &measure);
            lines.push((from, cut));
            from = cut;
        }
        line = (from < end).then_some((from, end));
    }
    lines.extend(line);
    lines
}

/// `wrap_hard_by` measured in the body face at `px`, the face `draw::text`
/// paints with.
pub fn wrap_hard(text: &str, max_w: u32, px: f32) -> Vec<(usize, usize)> {
    wrap_hard_by(text, i32::try_from(max_w).unwrap_or(i32::MAX), |t| measure_ttf(t, px))
}

/// The byte length of the longest prefix of `word` that fits `max_w`, or of
/// its first char when not even that fits, so every cut moves on.
fn piece(word: &str, max_w: i32, measure: &impl Fn(&str) -> i32) -> usize {
    match clip_by(word, max_w, measure).len() {
        0 => word.chars().next().map_or(word.len(), char::len_utf8),
        kept => kept,
    }
}
