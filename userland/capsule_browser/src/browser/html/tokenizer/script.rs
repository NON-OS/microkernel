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
use super::state::{TextMode, Tokenizer};
use super::token::Token;

impl<'a> Tokenizer<'a> {
    /// Script data: the text of a script up to the `</script` that really
    /// ends it, with NUL replaced. Where that is depends on the escape states
    /// `script_end` walks, so `"</"` in a string does not end anything and
    /// a `<!--<script>` block keeps its inner `</script>`.
    pub(super) fn script(&mut self) -> Option<Token<'a>> {
        if self.appropriate_end(self.pos) {
            self.pos += 2;
            self.mode = TextMode::Data;
            return self.tag(true);
        }
        let src: &'a str = self.src;
        let end = self.script_end();
        let text = replace_nul(&src[self.pos..end]);
        self.pos = end;
        Some(Token::Chars(text))
    }
}
