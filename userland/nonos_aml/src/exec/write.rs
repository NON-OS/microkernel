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

//! Writing to a place: Store's rules (section 19.6.132), and CopyObject's,
//! which replace the object instead of converting into it (section 19.6.27).

use super::place::Place;
use super::{Fault, Host, Machine};
use crate::load::Aml;
use crate::object::Object;

impl<H: Host> Machine<'_, H> {
    pub(crate) fn write(
        &mut self,
        aml: &mut Aml,
        place: Place,
        v: Object,
        at: u32,
        copy: bool,
    ) -> Result<(), Fault> {
        match place {
            Place::Null => Ok(()),
            Place::Debug => {
                self.host.debug(&v);
                Ok(())
            }
            Place::Local(s) => {
                *s.borrow_mut() = self.copy(&v, at)?;
                Ok(())
            }
            // An argument that holds a reference is stored through: that is
            // how a method returns values through RefOf arguments.
            Place::Arg(s) => {
                let held = s.borrow().clone();
                match held {
                    Object::Ref(r) if !copy => self.write_ref(aml, r, v, at, copy),
                    _ => {
                        *s.borrow_mut() = self.copy(&v, at)?;
                        Ok(())
                    }
                }
            }
            Place::Named(id) => self.write_named(aml, id, v, at, copy),
            Place::Ref(r) => self.write_ref(aml, r, v, at, copy),
        }
    }
}
