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

//! The hierarchical data set's links: pairs of a child's name and where its
//! node is, a path relative to the device (or, as firmware also writes it,
//! a reference). The scope is that of the object whose package lists the
//! link: the device for its _DSD, the device again for a node under it.

use super::set::set_in;
use super::uuid::HIERARCHICAL_DATA_UUID;
use super::value::{text, MAX};
use crate::load::Aml;
use crate::namespace::Id;
use crate::object::{Object, Ref};
use crate::platform::resolve_text;
use alloc::vec::Vec;

/// Each link's name and target, in order; a malformed link is skipped.
pub(super) fn links(dsd: &[Object]) -> Vec<(Vec<u8>, Object)> {
    let Some(Object::Package(l)) = set_in(dsd, &HIERARCHICAL_DATA_UUID) else {
        return Vec::new();
    };
    let one = |link: &Object| {
        let Object::Package(l) = link else { return None };
        let l = l.borrow();
        let (Some(Object::String(n)), Some(t)) = (l.first(), l.get(1)) else { return None };
        let name = text(&n.borrow());
        Some((name, t.clone()))
    };
    let l = l.borrow();
    l.iter().take(MAX).filter_map(one).collect()
}

/// The object a link names, from the package object `at` that lists it.
pub(super) fn target(aml: &Aml, at: Id, o: &Object) -> Option<Id> {
    let scope = aml.ns.get(at)?.parent;
    match o {
        Object::String(s) => resolve_text(&aml.ns, scope, &s.borrow()),
        Object::Ref(Ref::Named(id, gen)) if aml.ns.live(*id, *gen) => Some(*id),
        _ => None,
    }
}
