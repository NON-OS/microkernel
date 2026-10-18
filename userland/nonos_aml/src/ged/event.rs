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

//! One GED interrupt served (ACPI 6.5, 5.6.9.3): _EVT runs with the
//! interrupt's number, and the AML does what a GPE method would.

use super::find::Ged;
use crate::exec::Fault;
use crate::load::Aml;
use crate::object::Object;
use crate::platform::{child, Eval};
use alloc::vec;

#[derive(Debug)]
pub enum GedError {
    /// The interrupt is not one this GED's _CRS names.
    NotMine,
    /// The GED has no _EVT, so there is nothing to run.
    NoEvt,
    Fault(Fault),
}

pub fn event(aml: &mut Aml, eval: &mut Eval<'_>, ged: &Ged, gsiv: u32) -> Result<(), GedError> {
    if !ged.interrupts.iter().any(|i| i.gsiv == gsiv) {
        return Err(GedError::NotMine);
    }
    let args = vec![Object::Integer(u64::from(gsiv))];
    match child(aml, eval, ged.id, *b"_EVT", args) {
        None => Err(GedError::NoEvt),
        Some(r) => r.map(|_| ()).map_err(GedError::Fault),
    }
}
