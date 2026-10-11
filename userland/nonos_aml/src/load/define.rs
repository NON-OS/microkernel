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

//! Defining one object while its table loads.

use super::{Aml, LoadExec};
use crate::arena::Imm;
use crate::namespace::{Id, Kind, MethodInfo, NoteKind, Origin};
use crate::op::Op;

impl Aml {
    pub(super) fn define(&mut self, table: u16, node: u32, scope: Id, exec: &mut impl LoadExec) {
        let n = self.node(table, node);
        let kids = self.kids(table, node);
        let imm = |aml: &Aml, i: usize| match aml.node(table, kids[i]).imm {
            Imm::Int(v) => v as u8,
            _ => 0,
        };
        let (name_kid, kind) = match n.op {
            Op::SCOPE => return self.reopen(table, node, scope, exec),
            Op::DEVICE => (0, Kind::Device),
            Op::THERMAL_ZONE => (0, Kind::ThermalZone),
            Op::POWER_RES => (0, Kind::PowerResource),
            Op::PROCESSOR => (0, Kind::Processor),
            Op::METHOD => (0, Kind::Method(MethodInfo { flags: imm(self, 1) })),
            Op::NAME => (0, Kind::Name),
            Op::MUTEX => (0, Kind::Mutex),
            Op::EVENT => (0, Kind::Event),
            Op::OP_REGION => (0, Kind::OpRegion),
            Op::DATA_REGION => (0, Kind::DataRegion),
            // CreateXField names its field last.
            op if op.creates_buffer_field() => (kids.len() - 1, Kind::BufferField),
            Op::ALIAS => return self.alias(table, node, scope),
            Op::FIELD | Op::INDEX_FIELD | Op::BANK_FIELD => {
                return self.field_units(table, node, scope)
            }
            // External only tells the parser a method's arity; the object it
            // declares is defined, or not, by its own table.
            Op::EXTERNAL => return,
            Op::IF => return self.branch(table, node, scope, exec),
            _ => {
                if let Err(e) = exec.run(self, table, scope, node) {
                    self.notes.push(NoteKind::Exec, table, e.at);
                }
                return;
            }
        };
        let Some(path) = self.name_of(table, kids[name_kid]) else { return };
        let origin = Origin::Table { table, node };
        let Some(id) = self.create(table, n.at, scope, path, kind, origin) else { return };
        if kind.is_container() {
            self.walk(table, kids[kids.len() - 1], id, exec);
        }
    }
}
