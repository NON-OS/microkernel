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

//! A method body, parsed the first time it is called.

use super::{Fault, FaultKind, Host, Machine};
use crate::arena::Imm;
use crate::block::Block;
use crate::load::{Aml, ParseScopes};
use crate::namespace::Id;
use crate::parse;

impl<H: Host> Machine<'_, H> {
    /// The TERMS node of the Method at `node`. By the time a method runs every
    /// table is loaded, so each name its body calls can be counted; a body
    /// defined inside another method's run shares its parse with every other
    /// run of that definition.
    pub(crate) fn body(
        &mut self,
        aml: &mut Aml,
        table: u16,
        node: u32,
        id: Id,
    ) -> Result<u32, Fault> {
        if let Some(b) = aml.rt.bodies.get(&(table, node)) {
            return Ok(*b);
        }
        let deferred = aml.kid(table, node, 2);
        let n = aml.node(table, deferred);
        let Imm::Bytes(span) = n.imm else { return Err(self.fault(FaultKind::Type, n.at)) };
        let Aml { ns, tables, .. } = aml;
        let t = &mut tables[table as usize];
        let block = Block { bytes: &t.bytes };
        let mut scopes = ParseScopes::new(ns, id);
        let terms = parse::body(&block, &mut t.arena, &mut scopes, span)
            .map_err(|e| super::Fault::new(FaultKind::Parse(e.kind), table, e.at))?;
        aml.rt.bodies.insert((table, node), terms);
        Ok(terms)
    }
}
