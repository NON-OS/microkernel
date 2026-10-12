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

//! One property: a name and its value. A value is an integer, a string, a
//! reference to a named object, or a package of values, packages nesting
//! to `MAX_DEPTH`; what lies deeper, or is of another type, is dropped.

use crate::load::Aml;
use crate::namespace::Id;
use crate::object::{Object, Ref};
use alloc::vec::Vec;

/// Elements kept of a package, bytes of a string or name, properties of a
/// set: more than any device declares.
pub const MAX: usize = 64;
/// Packages within packages; a GPIO list (a package of a reference and
/// its arguments, in a package) needs two.
pub const MAX_DEPTH: u32 = 4;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Integer(u64),
    String(Vec<u8>),
    /// A named object (a GPIO controller, a power resource, a subnode).
    Reference(Id),
    Package(Vec<Value>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Property {
    pub name: Vec<u8>,
    pub value: Value,
}

pub(super) fn text(s: &[u8]) -> Vec<u8> {
    s.iter().copied().take_while(|c| *c != 0).take(MAX).collect()
}

fn value(aml: &Aml, o: &Object, depth: u32) -> Option<Value> {
    Some(match o {
        Object::Integer(v) => Value::Integer(*v),
        Object::String(s) => Value::String(text(&s.borrow())),
        Object::Ref(Ref::Named(id, gen)) if aml.ns.live(*id, *gen) => Value::Reference(*id),
        Object::Package(p) if depth < MAX_DEPTH => {
            let p = p.borrow();
            Value::Package(p.iter().take(MAX).filter_map(|e| value(aml, e, depth + 1)).collect())
        }
        _ => return None,
    })
}

/// A two-element package: the name, then the value.
pub(super) fn property(aml: &Aml, o: &Object) -> Option<Property> {
    let Object::Package(p) = o else { return None };
    let p = p.borrow();
    let Some(Object::String(name)) = p.first() else { return None };
    let name = text(&name.borrow());
    Some(Property { name, value: value(aml, p.get(1)?, 0)? })
}
