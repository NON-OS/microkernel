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

//! String, section 20.2.3: AsciiCharList, then NullChar.

use super::parser::Parser;
use super::Scopes;
use crate::arena::Span;
use crate::error::{Error, ErrorKind};
use crate::grammar::Cursor;

impl<S: Scopes> Parser<'_, S> {
    /// The characters, without the NUL. AsciiChar is 0x01-0x7F; a byte above
    /// that is refused rather than read in some code page the table never
    /// names.
    pub(super) fn string(&mut self, c: &mut Cursor<'_>) -> Result<Span, Error> {
        let at = c.pos;
        loop {
            match c.u8()? {
                0x00 => return Ok(Span { at, len: c.pos - 1 - at }),
                0x01..=0x7F => {}
                _ => return Err(Error::new(ErrorKind::String, c.pos - 1)),
            }
        }
    }
}
