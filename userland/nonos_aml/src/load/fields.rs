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

//! Field units: each NamedField of a Field, IndexField or BankField becomes an
//! object in the scope the field stands in, laid out from its FieldList.

use super::Aml;
use crate::arena::Imm;
use crate::grammar::NamePath;
use crate::namespace::{FieldUnit, Id, Kind, NoteKind, Origin};
use crate::op::Op;

impl Aml {
    pub(super) fn field_units(&mut self, table: u16, node: u32, scope: Id) {
        let kids = self.kids(table, node);
        let (list, flags) = (kids[kids.len() - 1], kids[kids.len() - 2]);
        let int = |aml: &Aml, n: u32| match aml.node(table, n).imm {
            Imm::Int(v) => v,
            _ => 0,
        };
        // FieldFlags bits 0-3 are the AccessType until an AccessField says
        // otherwise (section 20.2.5.2).
        let mut unit = FieldUnit {
            list: node,
            bit_offset: 0,
            bit_len: 0,
            access: [int(self, flags) as u8 & 0x0F, 0, 0],
            connection: None,
        };
        for el in self.kids(table, list) {
            let e = self.node(table, el);
            let v = int(self, el);
            let step = match e.op {
                Op::RESERVED_FIELD => v as u32,
                Op::ACCESS_FIELD => {
                    unit.access = [v as u8, (v >> 8) as u8, 0];
                    0
                }
                Op::EXTENDED_ACCESS_FIELD => {
                    unit.access = [v as u8, (v >> 8) as u8, (v >> 16) as u8];
                    0
                }
                Op::CONNECT_FIELD => {
                    unit.connection = Some(el);
                    0
                }
                _ => {
                    // A NamedField's node starts at its NameSeg.
                    let path = NamePath { root: false, up: 0, at: e.at, count: 1 };
                    let field = FieldUnit { bit_len: v as u32, ..unit };
                    let origin = Origin::Table { table, node: el };
                    self.create(table, e.at, scope, path, Kind::FieldUnit(field), origin);
                    v as u32
                }
            };
            let Some(next) = unit.bit_offset.checked_add(step) else {
                return self.notes.push(NoteKind::Layout, table, e.at);
            };
            unit.bit_offset = next;
        }
    }
}
