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

//! The interpreter as the loader's evaluator: load-time code runs in a frame
//! of its own, in the scope it stands in, with no arguments and fresh locals.

use super::{Fault, FaultKind, Flow, Frame, Host, Machine};
use crate::load::{Aml, LoadExec};
use crate::namespace::Id;

impl<H: Host> LoadExec for Machine<'_, H> {
    fn condition(
        &mut self,
        aml: &mut Aml,
        table: u16,
        scope: Id,
        node: u32,
    ) -> Result<bool, Fault> {
        self.frames.push(Frame::new(table, scope));
        let pred = aml.kid(table, node, 0);
        let r = self.int(aml, table, pred);
        self.frames.pop();
        Ok(r? != 0)
    }

    fn run(&mut self, aml: &mut Aml, table: u16, scope: Id, node: u32) -> Result<(), Fault> {
        self.frames.push(Frame::new(table, scope));
        let r = self.stmt(aml, table, node);
        self.frames.pop();
        match r? {
            Flow::Normal => Ok(()),
            // Return, Break or Continue at a table's top level has nowhere to go.
            _ => Err(Fault::new(FaultKind::Control, table, aml.node(table, node).at)),
        }
    }

    fn cut(&self) -> Option<u32> {
        self.cut
    }
}
