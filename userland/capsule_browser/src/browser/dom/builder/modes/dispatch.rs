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

use super::super::ops::chars::drop_first;
use super::super::ops::mode::Mode;
use super::super::ops::state::Builder;

impl Builder {
    /// Hand one token to the tree construction dispatcher (13.2.6).
    pub fn process(&mut self, mut token: Token) {
        if self.skip_lf {
            /*
             * The newline right after <pre>, <listing> or <textarea> is an
             * authoring convenience, not content.
             */
            self.skip_lf = false;
            if let Token::Chars(s) = &mut token {
                if s.starts_with('\n') {
                    drop_first(s);
                    if s.is_empty() {
                        return;
                    }
                }
            }
        }
        let comment = matches!(token, Token::Comment);
        if self.html_rules(&token) {
            self.in_mode(self.mode, token);
        } else {
            self.foreign(token);
        }
        /*
         * Comments before html, after body and after the document go to the
         * document or root, never between two runs of body text.
         */
        let outside = [Mode::Initial, Mode::BeforeHtml, Mode::AfterBody, Mode::AfterAfterBody];
        if comment && !self.open.is_empty() && !outside.contains(&self.mode) {
            self.mark_comment();
        }
    }
}
