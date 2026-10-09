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

use crate::browser::html::tokenizer::Token;

use super::super::ops::chars::split_space;
use super::super::ops::mode::Mode;
use super::super::ops::state::Builder;

impl Builder {
    /// The "in head noscript" mode, what a noscript in the head holds when
    /// scripting is off.
    pub(in super::super) fn in_head_noscript(&mut self, token: Token) {
        match token {
            Token::Doctype(_) => {}
            Token::Start(t) if t.name == "html" => self.in_body(Token::Start(t)),
            Token::End(t) if t.name == "noscript" => {
                self.pop();
                self.mode = Mode::InHead;
            }
            Token::Chars(s) => {
                let (space, rest) = split_space(s);
                self.insert_text(space);
                if !rest.is_empty() {
                    self.leave_noscript(Token::Chars(rest));
                }
            }
            Token::Comment => {}
            Token::Start(t)
                if matches!(
                    &*t.name,
                    "basefont" | "bgsound" | "link" | "meta" | "noframes" | "style"
                ) =>
            {
                self.in_head(Token::Start(t))
            }
            Token::Start(t) if matches!(&*t.name, "head" | "noscript") => {}
            Token::End(t) if t.name != "br" => {}
            other => self.leave_noscript(other),
        }
    }

    fn leave_noscript(&mut self, token: Token) {
        self.pop();
        self.reprocess(Mode::InHead, token);
    }
}
