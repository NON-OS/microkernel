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

//! The predefined method \_OSI (section 5.7.2).

use super::{Fault, FaultKind, Host, Machine};
use crate::load::Aml;
use crate::object::Object;

impl<H: Host> Machine<'_, H> {
    /// \_OSI(String): Ones if this build answers the string, Zero if not.
    pub(crate) fn osi(&mut self, aml: &Aml, args: &[Object], at: u32) -> Result<Object, Fault> {
        let Some(Object::String(s)) = args.first() else {
            return Err(self.fault(FaultKind::Type, at));
        };
        let yes = aml.rt.personality.answers(&s.borrow());
        Ok(Object::Integer(if yes { u64::MAX } else { 0 }))
    }
}
