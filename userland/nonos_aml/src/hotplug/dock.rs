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

//! Docking (ACPI 6.5, 6.3.2 and 6.3.5, recalled and unverified): a dock is
//! a device with `_DCK`. `_DCK(1)` docks and answers 1 on success; `_DCK(0)`
//! undocks, before the dock's `_EJ0`. A device elsewhere in the namespace
//! that leaves with the dock names it in its `_EJD`.

use crate::exec::Fault;
use crate::load::Aml;
use crate::namespace::{Id, Kind};
use crate::object::Object;
use crate::platform::{child, integer, resolve_text, scan, Eval};
use alloc::vec;
use alloc::vec::Vec;

pub fn is_dock(aml: &Aml, id: Id) -> bool {
    aml.ns.child(id, *b"_DCK").is_some()
}

/// Whether `_DCK(on)` reported success; None when there is no `_DCK`.
pub fn dock(aml: &mut Aml, eval: &mut Eval<'_>, id: Id, on: bool) -> Option<Result<bool, Fault>> {
    let r = child(aml, eval, id, *b"_DCK", vec![Object::Integer(u64::from(on))])?;
    Some(r.map(|o| integer(&o) == Some(1)))
}

fn ejd(aml: &mut Aml, eval: &mut Eval<'_>, id: Id) -> Option<Id> {
    let Object::String(s) = child(aml, eval, id, *b"_EJD", Vec::new())?.ok()? else {
        return None;
    };
    let scope = aml.ns.get(id)?.parent;
    let text = s.borrow().clone();
    resolve_text(&aml.ns, scope, &text)
}

/// Every device whose `_EJD` names `dock`, in definition order.
pub fn dependents(aml: &mut Aml, eval: &mut Eval<'_>, dock: Id) -> Vec<Id> {
    let with_ejd = scan(aml, &|aml, id| {
        aml.ns.get(id).is_some_and(|e| e.kind == Kind::Device)
            && aml.ns.child(id, *b"_EJD").is_some()
    });
    with_ejd.into_iter().filter(|id| ejd(aml, eval, *id) == Some(dock)).collect()
}
