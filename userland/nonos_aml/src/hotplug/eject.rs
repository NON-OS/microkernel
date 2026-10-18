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

//! `_EJ0` (ACPI 6.5, 6.3.3): firmware removes the device once OSPM is done
//! with it. The argument is 1, a hot eject (recalled, unverified); the
//! caller checks `_STA` after, since the device is gone only once it says
//! so.

use crate::exec::Fault;
use crate::load::Aml;
use crate::namespace::Id;
use crate::object::Object;
use crate::platform::{child, Eval};
use alloc::vec;

const HOT_EJECT: u64 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EjectError {
    /// The device has no `_EJ0`: firmware cannot remove it.
    NotEjectable,
    Fault(Fault),
}

pub fn ejectable(aml: &Aml, id: Id) -> bool {
    aml.ns.child(id, *b"_EJ0").is_some()
}

pub fn eject(aml: &mut Aml, eval: &mut Eval<'_>, id: Id) -> Result<(), EjectError> {
    let arg = vec![Object::Integer(HOT_EJECT)];
    match child(aml, eval, id, *b"_EJ0", arg) {
        None => Err(EjectError::NotEjectable),
        Some(r) => r.map(|_| ()).map_err(EjectError::Fault),
    }
}
