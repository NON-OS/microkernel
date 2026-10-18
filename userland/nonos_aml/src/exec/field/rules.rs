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

//! The rules every field access keeps: its bank selected, and the Global
//! Lock held if its LockRule asks.

use super::index_bank::Ctx;
use super::read::LOCK;
use crate::exec::{Fault, Host, Machine};
use crate::load::Aml;
use crate::object::Object;

impl<H: Host> Machine<'_, H> {
    /// A BankField selects its bank before each access.
    pub(crate) fn select_bank(&mut self, aml: &mut Aml, ctx: &Ctx, at: u32) -> Result<(), Fault> {
        match ctx.bank {
            Some((bank, value)) => self.field_write(aml, bank, Object::Integer(value), at),
            None => Ok(()),
        }
    }

    /// Runs `f` holding the Global Lock if `flags` asks for it.
    pub(crate) fn locked<T>(
        &mut self,
        aml: &mut Aml,
        flags: u8,
        at: u32,
        f: impl FnOnce(&mut Self, &mut Aml) -> Result<T, Fault>,
    ) -> Result<T, Fault> {
        if flags & LOCK == 0 {
            return f(self, aml);
        }
        self.global_lock(aml, true, at)?;
        let r = f(self, aml);
        self.global_lock(aml, false, at)?;
        r
    }
}
