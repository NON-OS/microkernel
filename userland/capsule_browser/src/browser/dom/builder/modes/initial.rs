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

use super::super::super::quirks::Quirks;
use super::super::doc::quirks::quirks_of;
use super::super::ops::chars::split_space;
use super::super::ops::mode::Mode;
use super::super::ops::state::Builder;

impl Builder {
    /// The "initial" mode: the doctype, if any, picks the document's mode.
    pub(in super::super) fn initial(&mut self, token: Token) {
        match token {
            Token::Chars(s) => {
                let (_, rest) = split_space(s);
                if !rest.is_empty() {
                    self.dom.quirks = Quirks::Full;
                    self.reprocess(Mode::BeforeHtml, Token::Chars(rest));
                }
            }
            Token::Comment => {}
            Token::Doctype(d) => {
                self.dom.quirks = quirks_of(&d);
                self.mode = Mode::BeforeHtml;
            }
            other => {
                self.dom.quirks = Quirks::Full;
                self.reprocess(Mode::BeforeHtml, other);
            }
        }
    }
}
