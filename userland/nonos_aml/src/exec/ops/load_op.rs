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

//! Load and LoadTable (sections 19.6.74 and 19.6.75): a definition block
//! read from a region, a field or a buffer, or found by its ids among the
//! platform's tables, and loaded while the evaluation runs.

use crate::exec::{Fault, Host, Machine};
use crate::load::Aml;
use crate::namespace::ROOT;
use crate::object::Object;
use crate::op::Op;

impl<H: Host> Machine<'_, H> {
    /// Ones and the DDBHandle stored to the target if the table loaded;
    /// Zero if it was refused, which the table's own checks decide.
    pub(crate) fn load_op(
        &mut self,
        aml: &mut Aml,
        t: u16,
        node: u32,
        op: Op,
        at: u32,
    ) -> Result<Object, Fault> {
        if op == Op::LOAD_TABLE {
            return self.load_table_op(aml, t, node, at);
        }
        let (src, target) = (aml.kid(t, node, 0), aml.kid(t, node, 1));
        let id = self.lookup(aml, t, src)?;
        let bytes = self.table_bytes(aml, id, at)?;
        let w = self.width(aml, t);
        if !self.host.admit_table(&bytes) {
            return Ok(Object::Integer(0));
        }
        match aml.load_table_at(bytes, ROOT, self) {
            Ok(index) => {
                self.announce_loaded(aml);
                self.result(aml, t, target, Object::Table(index), at)?;
                Ok(Object::Integer(w.ones()))
            }
            Err(_) => Ok(Object::Integer(0)),
        }
    }
}
