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

//! Running one of a device's or a power resource's own methods, when it
//! has it: _ON, _OFF, _PS0, _PS3. A missing one is no fault; the
//! specification makes each optional where it allows the step at all.

use crate::exec::Fault;
use crate::load::Aml;
use crate::namespace::{Id, Seg};
use crate::platform::Eval;
use alloc::vec::Vec;

pub(super) fn run(aml: &mut Aml, eval: &mut Eval<'_>, at: Id, seg: Seg) -> Result<(), Fault> {
    match aml.ns.child(at, seg) {
        Some(id) => eval(aml, id, Vec::new()).map(|_| ()),
        None => Ok(()),
    }
}
