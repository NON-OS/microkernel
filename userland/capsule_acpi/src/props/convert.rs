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

//! The interpreter's tree in the wire's terms: a reference becomes the
//! named object's namespace path, the only name a driver can use for it.

use super::wire::{Prop, Tree, Val};
use crate::boot::path;
use nonos_aml::load::Aml;
use nonos_aml::props::{Node, Property, Value};

fn val(aml: &Aml, v: &Value) -> Val {
    match v {
        Value::Integer(i) => Val::Int(*i),
        Value::String(s) => Val::Str(s.clone()),
        Value::Reference(id) => Val::Ref(path(aml, *id).into_bytes()),
        Value::Package(e) => Val::Pkg(e.iter().map(|x| val(aml, x)).collect()),
    }
}

fn prop(aml: &Aml, p: &Property) -> Prop {
    Prop { name: p.name.clone(), val: val(aml, &p.value) }
}

pub fn convert(aml: &Aml, n: &Node) -> Tree {
    Tree {
        name: n.name.clone(),
        props: n.properties.iter().map(|p| prop(aml, p)).collect(),
        children: n.children.iter().map(|c| convert(aml, c)).collect(),
    }
}
