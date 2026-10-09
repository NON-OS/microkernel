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

use crate::browser::html::tokenizer::{Tag, TextMode, Token};

use super::super::ops::mode::Mode;
use super::super::ops::state::Builder;

impl Builder {
    /// The "text" mode: the contents of a script, style, title, textarea or
    /// other raw text element, up to its end tag.
    pub(in super::super) fn text_mode(&mut self, token: Token) {
        match token {
            Token::Chars(s) => self.insert_text(s),
            Token::Eof => {
                self.pop();
                self.reprocess(self.orig, Token::Eof);
            }
            Token::End(_) => {
                self.pop();
                self.mode = self.orig;
            }
            /* The tokenizer's raw text states produce nothing else. */
            Token::Null | Token::Start(_) | Token::Comment | Token::Doctype(_) => {}
        }
    }

    /// The generic raw text and RCDATA element parsing algorithms (13.2.6.2),
    /// which script shares with its own tokenizer state.
    pub(in super::super) fn raw_element(&mut self, t: Tag, state: TextMode) {
        self.insert_html(t);
        self.switch_to = Some(state);
        self.orig = self.mode;
        self.mode = Mode::Text;
    }

    /// A textarea is RCDATA like a title, but a newline straight after its
    /// start tag is dropped.
    pub(in super::super) fn textarea(&mut self, t: Tag) {
        self.frameset_ok = false;
        self.skip_lf = true;
        self.raw_element(t, TextMode::Rcdata);
    }
}
