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

//! An operator's result, to its target and back to its caller.

use super::{Fault, Host, Machine};
use crate::load::Aml;
use crate::object::Object;

impl<H: Host> Machine<'_, H> {
    /// Stores an operator's result to its target, if it has one, and returns
    /// it.
    pub(crate) fn result(
        &mut self,
        aml: &mut Aml,
        t: u16,
        target: u32,
        v: Object,
        at: u32,
    ) -> Result<Object, Fault> {
        let place = self.place(aml, t, target)?;
        self.write(aml, place, v.clone(), at, false)?;
        Ok(v)
    }
}
