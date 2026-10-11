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

//! A mutex's sync level.

use crate::exec::{Fault, FaultKind, Host, Machine};
use crate::load::Aml;
use crate::namespace::{Id, Kind, Origin};

impl<H: Host> Machine<'_, H> {
    /// A Mutex's SyncLevel is its SyncFlags' low four bits; the Global Lock
    /// is at level 0.
    pub(crate) fn sync_level(&self, aml: &Aml, m: Id, at: u32) -> Result<u8, Fault> {
        let e = aml.ns.get(m).ok_or_else(|| self.fault(FaultKind::Stale, at))?;
        match (e.kind, e.origin) {
            (Kind::Mutex, Origin::Table { table, node }) => {
                Ok(aml.imm(table, aml.kid(table, node, 1)) as u8 & 0x0F)
            }
            (Kind::Mutex, Origin::Predefined(_)) => Ok(0),
            _ => Err(self.fault(FaultKind::Type, at)),
        }
    }
}
