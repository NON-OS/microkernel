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

use alloc::borrow::Cow;
use alloc::string::String;

/* A preserved line as one run, or split into runs of spaces and of other
 * text where the line may wrap after its spaces. */
pub(in super::super) fn runs(line: &str, whole: bool) -> impl Iterator<Item = &str> {
    let mut rest = (!line.is_empty()).then_some(line);
    core::iter::from_fn(move || {
        let r = rest?;
        let sp = r.starts_with(' ');
        let cut = r.find(|ch: char| (ch == ' ') != sp).unwrap_or(r.len());
        let end = if whole { r.len() } else { cut };
        rest = (end < r.len()).then(|| &r[end..]);
        Some(&r[..end])
    })
}

/* A word cut after each hyphen, slash or dash that a letter follows, the
 * places a line may wrap inside a word ("client/server", "well-known").
 * One piece when wrapping is off; none for an empty word. */
pub(in super::super) fn soft_pieces(w: &str, wrap: bool) -> impl Iterator<Item = &str> {
    let mut rest = (!w.is_empty()).then_some(w);
    core::iter::from_fn(move || {
        let r = rest?;
        let mut seen_text = false;
        let mut cut = r.len();
        let mut it = r.char_indices().peekable();
        while let (true, Some((i, ch))) = (wrap, it.next()) {
            let next_letter = it.peek().is_some_and(|&(_, n)| n.is_alphabetic());
            if seen_text && next_letter && matches!(ch, '-' | '/' | '\u{2013}' | '\u{2014}') {
                cut = i + ch.len_utf8();
                break;
            }
            seen_text |= ch.is_alphanumeric();
        }
        rest = (cut < r.len()).then(|| &r[cut..]);
        Some(&r[..cut])
    })
}

/* Tab stops sit every eight columns, as tab-size's initial value puts them. */
const TAB: usize = 8;

/* A preserved line with each tab widened to the next tab stop. */
pub(in super::super) fn expand_tabs(line: &str) -> Cow<'_, str> {
    if !line.contains('\t') {
        return Cow::Borrowed(line);
    }
    let (mut out, mut col) = (String::with_capacity(line.len() + TAB), 0usize);
    for ch in line.chars() {
        let n = if ch == '\t' { TAB - col % TAB } else { 1 };
        out.extend(core::iter::repeat_n(if ch == '\t' { ' ' } else { ch }, n));
        col += n;
    }
    Cow::Owned(out)
}
