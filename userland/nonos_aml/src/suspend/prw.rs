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

//! A device's _PRW (ACPI 6.5, 7.3.13): its wake GPE (None for the GPE
//! block form, whose event lies outside the FADT's blocks), the deepest
//! sleeping state it can wake the system from, and the power resources
//! that must be on for it to do so.

use crate::load::Aml;
use crate::namespace::Id;
use crate::object::Object;
use crate::platform::{child, references, Eval};
use alloc::vec::Vec;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Prw {
    pub gpe: Option<u16>,
    pub deepest: u8,
    pub resources: Vec<Id>,
}

/// None when the device has no _PRW, it faults, or it is not a package of
/// at least an event and a sleeping state.
pub fn read_prw(aml: &mut Aml, eval: &mut Eval<'_>, dev: Id) -> Option<Prw> {
    let o = child(aml, eval, dev, *b"_PRW", Vec::new())?.ok()?;
    let (gpe, deepest) = {
        let Object::Package(p) = &o else { return None };
        let v = p.borrow();
        let gpe = match v.first()? {
            Object::Integer(n) => Some(u16::try_from(*n).ok()?),
            Object::Package(_) => None,
            _ => return None,
        };
        let Some(Object::Integer(s)) = v.get(1) else { return None };
        (gpe, u8::try_from(*s).ok()?)
    };
    Some(Prw { gpe, deepest, resources: references(aml, &o) })
}
