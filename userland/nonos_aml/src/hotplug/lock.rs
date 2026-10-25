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

//! `_LCK` and `_RMV` (ACPI 6.5, 6.3.4 and 6.3.6; recalled, unverified).
//! `_LCK(1)` locks a device in place, so it cannot be taken out while it
//! is in use, and `_LCK(0)` unlocks it; on a lock, the method returns only
//! once the device is locked. `_RMV` answers 1 for a device the user may
//! take out with no `_EJ0`, a surprise-removable one.

use crate::exec::Fault;
use crate::load::Aml;
use crate::namespace::Id;
use crate::object::Object;
use crate::platform::{child, integer, Eval};
use alloc::vec;
use alloc::vec::Vec;

pub fn lockable(aml: &Aml, id: Id) -> bool {
    aml.ns.child(id, *b"_LCK").is_some()
}

/// None when there is no `_LCK`.
pub fn lock(aml: &mut Aml, eval: &mut Eval<'_>, id: Id, on: bool) -> Option<Result<(), Fault>> {
    let r = child(aml, eval, id, *b"_LCK", vec![Object::Integer(u64::from(on))])?;
    Some(r.map(|_| ()))
}

/// Only an `_RMV` that answers exactly 1; none, a fault or anything else
/// is not removable.
pub fn removable(aml: &mut Aml, eval: &mut Eval<'_>, id: Id) -> bool {
    let r = child(aml, eval, id, *b"_RMV", Vec::new());
    matches!(r, Some(Ok(o)) if integer(&o) == Some(1))
}
