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

use super::state::Tokenizer;
use super::token::Token;

/// Where a comment stands after each character (13.2.5.43 to 13.2.5.52):
/// inside it, after one dash, after two, and after "--!".
#[derive(Clone, Copy)]
enum At {
    Text,
    Dash,
    DashDash,
    Bang,
}

impl<'a> Tokenizer<'a> {
    /// A comment, the position just past "<!--". It ends at "-->" or "--!>",
    /// and "<!-->" and "<!--->" are complete empty comments; an unterminated
    /// one runs to the end of the input.
    pub(super) fn comment(&mut self) -> Token<'a> {
        let b = self.src.as_bytes();
        let start = self.pos;
        if b.get(start) == Some(&b'>') {
            self.pos = start + 1;
            return Token::Comment;
        }
        if b.get(start) == Some(&b'-') && b.get(start + 1) == Some(&b'>') {
            self.pos = start + 2;
            return Token::Comment;
        }
        let mut at = At::Text;
        let mut i = start;
        while i < b.len() {
            if let At::Text = at {
                match b[i..].iter().position(|&c| c == b'-') {
                    Some(n) => i += n,
                    None => break,
                }
            }
            let c = b[i];
            i += 1;
            at = match (at, c) {
                (At::Text, _) | (At::Bang, b'-') => At::Dash,
                (At::Dash, b'-') => At::DashDash,
                (At::DashDash, b'-') => At::DashDash,
                (At::DashDash, b'!') => At::Bang,
                (At::DashDash, b'>') | (At::Bang, b'>') => {
                    self.pos = i;
                    return Token::Comment;
                }
                _ => At::Text,
            };
        }
        self.pos = b.len();
        Token::Comment
    }
}
