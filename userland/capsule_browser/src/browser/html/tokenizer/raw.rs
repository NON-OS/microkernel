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

use crate::browser::html::entity::charref;

use super::run::{may_start_ref, Run};
use super::state::{TextMode, Tokenizer};
use super::token::Token;

impl<'a> Tokenizer<'a> {
    /// RCDATA (title, textarea) and RAWTEXT (style, xmp, iframe, noembed,
    /// noframes): text up to the appropriate end tag. Only RCDATA decodes
    /// character references. Any other "</" is text, including one whose
    /// name the input cuts off, so `</title` at the end of a page is shown.
    pub(super) fn raw(&mut self, rcdata: bool) -> Option<Token<'a>> {
        if self.appropriate_end(self.pos) {
            self.pos += 2;
            self.mode = TextMode::Data;
            return self.tag(true);
        }
        let src: &'a str = self.src;
        let b = src.as_bytes();
        let mut run = Run::new(self.pos);
        loop {
            let from = self.pos;
            let n =
                b[from..].iter().position(|&c| c == b'<' || c == b'\0' || (rcdata && c == b'&'));
            let stop = n.map_or(b.len(), |n| from + n);
            self.pos = stop;
            match b.get(stop) {
                None => break,
                Some(b'<') if self.appropriate_end(stop) => break,
                Some(b'&') if may_start_ref(b.get(stop + 1)) => {
                    let rest = &src[stop + 1..];
                    self.pos += run.decode(src, stop, |out| 1 + charref(rest, false, out));
                }
                Some(b'\0') => {
                    self.pos += run.decode(src, stop, |out| {
                        out.push('\u{FFFD}');
                        1
                    });
                }
                Some(_) => self.pos += 1,
            }
        }
        Some(Token::Chars(run.finish(src, self.pos)))
    }
}
