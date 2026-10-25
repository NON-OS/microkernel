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

//! FieldList, section 20.2.5.2: named and reserved fields, access changes and
//! connections, to the end of the field's package.

use super::parser::Parser;
use super::{Pos, Scopes};
use crate::arena::Imm;
use crate::error::{Error, ErrorKind};
use crate::grammar::{is_lead_char, is_name_char, pkg_value, Cursor};
use crate::op::Op;

impl<S: Scopes> Parser<'_, S> {
    pub(super) fn fields(&mut self, c: &mut Cursor<'_>) -> Result<u32, Error> {
        let (at, mark) = (c.pos, self.arena.mark());
        while !c.done() {
            let f = self.field(c)?;
            self.arena.push_kid(f);
        }
        self.arena.finish(Op::FIELD_LIST, at, Imm::None, mark)
    }

    /// One FieldElement. A NamedField's node starts at its NameSeg and holds
    /// its width in bits; the access fields hold their bytes packed low to
    /// high: AccessType, AccessAttrib, then AccessLength.
    fn field(&mut self, c: &mut Cursor<'_>) -> Result<u32, Error> {
        let (at, mark) = (c.pos, self.arena.mark());
        let (op, imm) = match c.u8()? {
            0x00 => (Op::RESERVED_FIELD, Imm::Int(pkg_value(c)? as u64)),
            0x01 => (Op::ACCESS_FIELD, Imm::Int(c.le(2)?)),
            0x03 => (Op::EXTENDED_ACCESS_FIELD, Imm::Int(c.le(3)?)),
            0x02 => {
                // ConnectField: a NameString, or a BufferData resource.
                let k = match c.peek() {
                    Some(b) if b == Op::BUFFER.0 as u8 => self.term(c, Pos::Data)?,
                    _ => self.name_only(c)?.0,
                };
                self.arena.push_kid(k);
                (Op::CONNECT_FIELD, Imm::None)
            }
            b if is_lead_char(b) => {
                let seg = c.take(3)?;
                let rest = &c.bytes[seg as usize..c.pos as usize];
                if let Some(i) = rest.iter().position(|b| !is_name_char(*b)) {
                    return Err(Error::new(ErrorKind::Name, seg + i as u32));
                }
                (Op::NAMED_FIELD, Imm::Int(pkg_value(c)? as u64))
            }
            b => return Err(Error::new(ErrorKind::Opcode(b as u16), at)),
        };
        self.arena.finish(op, at, imm, mark)
    }
}
