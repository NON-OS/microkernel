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

//! Where a field's accesses go: its region, or its index and data fields.

use super::index_bank::Target;
use crate::exec::{Fault, Host, Machine};
use crate::load::Aml;
use crate::namespace::Id;
use crate::op::Op;

impl<H: Host> Machine<'_, H> {
    pub(crate) fn target(
        &mut self,
        aml: &mut Aml,
        t: u16,
        list: u32,
        k: &[u32],
        at: u32,
    ) -> Result<(Target, Option<(Id, u64)>), Fault> {
        let first = self.lookup(aml, t, k[0])?;
        match aml.node(t, list).op {
            Op::INDEX_FIELD => {
                Ok((Target::Index { index: first, data: self.lookup(aml, t, k[1])? }, None))
            }
            op => {
                let region = self.region(aml, first, at)?;
                let bank = match op {
                    Op::BANK_FIELD => Some((self.lookup(aml, t, k[1])?, self.int(aml, t, k[2])?)),
                    _ => None,
                };
                Ok((Target::Region(first, region), bank))
            }
        }
    }
}
