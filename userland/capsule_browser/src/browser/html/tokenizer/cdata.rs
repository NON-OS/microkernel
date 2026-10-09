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

use super::run::replace_nul;
use super::state::Tokenizer;
use super::token::Token;

impl<'a> Tokenizer<'a> {
    /// A CDATA section in foreign content, the position just past
    /// "<![CDATA[": its text, verbatim, up to "]]>" or the end of the input.
    /// A NUL becomes U+FFFD here, which is what foreign content does with it.
    /// An empty section produces no token.
    pub(super) fn cdata(&mut self) -> Option<Token<'a>> {
        let src: &'a str = self.src;
        let (end, next) = match src[self.pos..].find("]]>") {
            Some(n) => (self.pos + n, self.pos + n + 3),
            None => (src.len(), src.len()),
        };
        let text = replace_nul(&src[self.pos..end]);
        self.pos = next;
        if text.is_empty() {
            return None;
        }
        Some(Token::Chars(text))
    }
}
