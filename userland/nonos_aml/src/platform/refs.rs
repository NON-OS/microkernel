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

//! The objects a package names: _PR0's power resources, _ALx's fans, _PSL's
//! processors. A name in a package evaluates to a reference to what it
//! names (ACPI 6.5, 19.6.102); one the namespace no longer holds, or that
//! never resolved and stayed text, names nothing.

use crate::load::Aml;
use crate::namespace::{Id, Seg};
use crate::object::{Object, Ref};
use crate::platform::Eval;
use alloc::vec::Vec;

pub fn references(aml: &Aml, o: &Object) -> Vec<Id> {
    let Object::Package(p) = o else { return Vec::new() };
    let named = |e: &Object| match e {
        Object::Ref(Ref::Named(id, gen)) if aml.ns.live(*id, *gen) => aml.ns.unalias(*id),
        _ => None,
    };
    p.borrow().iter().filter_map(named).collect()
}

/// The objects the package `seg` under `parent` names; none when it is
/// absent or fails.
pub fn listed(aml: &mut Aml, eval: &mut Eval<'_>, parent: Id, seg: Seg) -> Vec<Id> {
    let Some(id) = aml.ns.child(parent, seg) else { return Vec::new() };
    match eval(aml, id, Vec::new()) {
        Ok(o) => references(aml, &o),
        Err(_) => Vec::new(),
    }
}
