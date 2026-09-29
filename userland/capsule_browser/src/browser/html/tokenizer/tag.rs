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

use alloc::vec::Vec;

use super::state::Tokenizer;
use super::token::{Tag, Token};

impl<'a> Tokenizer<'a> {
    /// A start or end tag, the position at the first letter of its name. It
    /// ends at the first ">" outside a quoted value, however long; None when
    /// the input ends inside it, which drops the whole tag.
    pub(super) fn tag(&mut self, end: bool) -> Option<Token<'a>> {
        let mut tag = Tag { name: self.tag_name(), attrs: Vec::new(), self_closing: false };
        self.tag_rest(&mut tag, !end)?;
        if end {
            /* An end tag's attributes were not kept; its slash means nothing. */
            tag.self_closing = false;
            return Some(Token::End(tag));
        }
        self.last_start.clear();
        self.last_start.push_str(&tag.name);
        Some(Token::Start(tag))
    }

    /// The attributes and the close (13.2.5.32 to 13.2.5.40). A "/" not
    /// followed by ">" is dropped and reading goes on.
    fn tag_rest(&mut self, tag: &mut Tag, keep: bool) -> Option<()> {
        loop {
            self.skip_ws();
            match self.peek()? {
                b'>' => {
                    self.pos += 1;
                    return Some(());
                }
                b'/' => {
                    self.pos += 1;
                    if self.peek()? == b'>' {
                        self.pos += 1;
                        tag.self_closing = true;
                        return Some(());
                    }
                }
                _ => self.attribute(tag, keep)?,
            }
        }
    }
}
