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

//! The interpreter's state: the namespace and the tables that define it.

use super::order::in_order;
use super::{LoadExec, ParseScopes, Table};
use crate::arena::{Arena, Limits};
use crate::block::Block;
use crate::error::{Error, ErrorKind};
use crate::exec::Runtime;
use crate::namespace::{Id, Namespace, Notes, ROOT};
use crate::parse;
use alloc::vec::Vec;

#[derive(Debug, Default)]
pub struct Aml {
    pub ns: Namespace,
    pub tables: Vec<Table>,
    pub notes: Notes,
    pub limits: Limits,
    pub rt: Runtime,
}

impl Aml {
    pub fn new(limits: Limits) -> Self {
        Self { limits, ..Self::default() }
    }

    /// Loads one definition block and returns its index: the DSDT first and
    /// only first, then SSDTs and PSDTs (section 5.2.11.2). A table refused
    /// here adds nothing, save a DSDT cut short: it cannot be taken out, and
    /// is refused so that none of it is served.
    pub fn load_table(&mut self, bytes: Vec<u8>, exec: &mut impl LoadExec) -> Result<u16, Error> {
        self.load_table_at(bytes, ROOT, exec)
    }

    /// As `load_table`, into `scope` (LoadTable's RootPathString, 19.6.75); OEMx tables too.
    pub fn load_table_at(
        &mut self,
        bytes: Vec<u8>,
        scope: Id,
        exec: &mut impl LoadExec,
    ) -> Result<u16, Error> {
        let block = Block::new(&bytes, self.limits.max_table)?;
        let ordered = in_order(block.signature(), self.tables.is_empty());
        ordered.then_some(()).ok_or(Error::new(ErrorKind::Signature, 0))?;
        let index = u16::try_from(self.tables.len()).map_err(|_| Error::new(ErrorKind::Size, 0))?;
        let mut arena = Arena::new(self.limits);
        let root = parse::load(&block, &mut arena, &mut ParseScopes::new(&self.ns, scope))?;
        self.tables.push(Table { bytes, arena, root });
        let (saved, cut) = (self.ns.journal_pause(), exec.cut());
        self.walk(index, root, scope, exec);
        self.ns.journal_resume(saved);
        // Cut short by the budget: taken back out whole (the DSDT cannot be).
        if let (None, Some(at)) = (cut, exec.cut()) {
            self.unload(index);
            return Err(Error::new(ErrorKind::Budget, at));
        }
        Ok(index)
    }
}
