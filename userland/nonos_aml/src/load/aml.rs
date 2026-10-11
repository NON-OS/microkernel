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

use super::{LoadExec, ParseScopes, Table};
use crate::arena::{Arena, Limits};
use crate::block::Block;
use crate::error::{Error, ErrorKind};
use crate::exec::Runtime;
use crate::namespace::{Namespace, Notes, ROOT};
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
        let (ns, notes, rt) = (Namespace::new(), Notes::default(), Runtime::default());
        Self { ns, tables: Vec::new(), notes, limits, rt }
    }

    /// Loads one definition block and returns its index. The DSDT must come
    /// first and only first; SSDTs and PSDTs follow (section 5.2.11.2). A
    /// table refused here adds nothing to the namespace.
    pub fn load_table(&mut self, bytes: Vec<u8>, exec: &mut impl LoadExec) -> Result<u16, Error> {
        let block = Block::new(&bytes, self.limits.max_table)?;
        let first = self.tables.is_empty();
        let in_order = match &block.signature() {
            b"DSDT" => first,
            b"SSDT" | b"PSDT" => !first,
            _ => false,
        };
        if !in_order {
            return Err(Error::new(ErrorKind::Signature, 0));
        }
        let index = u16::try_from(self.tables.len()).map_err(|_| Error::new(ErrorKind::Size, 0))?;
        let mut arena = Arena::new(self.limits);
        let root = parse::load(&block, &mut arena, &mut ParseScopes::new(&self.ns, ROOT))?;
        self.tables.push(Table { bytes, arena, root });
        self.walk(index, root, ROOT, exec);
        Ok(index)
    }
}
