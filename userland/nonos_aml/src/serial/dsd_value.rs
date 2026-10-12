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

//! One _DSD property: a name and its value, at most one list deep.

use super::dsd::MAX;
use crate::load::Aml;
use crate::namespace::Id;
use crate::object::{Object, Ref};
use alloc::vec::Vec;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Integer(u64),
    String(Vec<u8>),
    /// A named object (a GPIO controller, a power resource).
    Reference(Id),
    /// One level of values: a list, or a reference with its arguments.
    List(Vec<Value>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Property {
    pub name: Vec<u8>,
    pub value: Value,
}

fn value(aml: &Aml, o: &Object, deep: bool) -> Option<Value> {
    Some(match o {
        Object::Integer(v) => Value::Integer(*v),
        Object::String(s) => Value::String(s.borrow().iter().copied().take(MAX).collect()),
        Object::Ref(Ref::Named(id, gen)) if aml.ns.live(*id, *gen) => Value::Reference(*id),
        Object::Package(p) if deep => {
            let p = p.borrow();
            Value::List(p.iter().take(MAX).filter_map(|e| value(aml, e, false)).collect())
        }
        _ => return None,
    })
}

pub(super) fn property(aml: &Aml, o: &Object) -> Option<Property> {
    let Object::Package(p) = o else { return None };
    let p = p.borrow();
    let Some(Object::String(name)) = p.first() else { return None };
    let name = name.borrow().iter().copied().take_while(|c| *c != 0).take(MAX).collect();
    Some(Property { name, value: value(aml, p.get(1)?, true)? })
}
