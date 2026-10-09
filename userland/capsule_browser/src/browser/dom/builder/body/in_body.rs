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

use crate::browser::html::tokenizer::Token;

use super::super::ops::chars::all_space;
use super::super::ops::mode::Mode;
use super::super::ops::scope::Scope;
use super::super::ops::state::Builder;

impl Builder {
    /// The "in body" mode (13.2.6.4.7).
    pub(in super::super) fn in_body(&mut self, token: Token) {
        match token {
            Token::Chars(s) => self.body_text(s),
            Token::Null | Token::Comment | Token::Doctype(_) => {}
            Token::Start(t) => self.body_start(t),
            Token::End(t) => self.body_end(t),
            Token::Eof => self.body_eof(),
        }
    }

    /// Characters in body content, which reopen formatting first.
    pub(in super::super) fn body_text(&mut self, s: Cow<'_, str>) {
        self.reconstruct();
        if !all_space(&s) {
            self.frameset_ok = false;
        }
        self.insert_text(s);
    }

    fn body_eof(&mut self) {
        if self.tmpl.is_empty() {
            self.stopped = true;
        } else {
            self.in_template(Token::Eof);
        }
    }

    /// </body> and </html>: on to "after body", which puts whatever follows
    /// back into the body rather than after it.
    pub(in super::super) fn body_close(&mut self, token: Token) {
        if self.in_scope("body", Scope::Default) {
            self.mode = Mode::AfterBody;
            if matches!(&token, Token::End(t) if t.name == "html") {
                self.process(token);
            }
        }
    }
}
