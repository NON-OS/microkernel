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

//! A device's _DEP: a package of references to the objects it depends on.
//! Firmware writes them as references, and some as path strings, which
//! resolve from the device's scope. One that names nothing is kept as
//! dangling, never dropped, so the device that names it can be flagged.

use crate::load::Aml;
use crate::namespace::Id;
use crate::object::{Object, Ref};
use crate::platform::{child, resolve_text, Eval};
use alloc::vec::Vec;

/// Entries read of one _DEP; more than any device lists.
const MAX: usize = 32;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dep {
    On(Id),
    Dangling,
}

/// Empty when the device has no _DEP, or it fails or is no package.
pub fn dependencies(aml: &mut Aml, eval: &mut Eval<'_>, dev: Id) -> Vec<Dep> {
    let Some(Ok(Object::Package(p))) = child(aml, eval, dev, *b"_DEP", Vec::new()) else {
        return Vec::new();
    };
    let p = p.borrow();
    let scope = aml.ns.get(dev).map(|e| e.parent);
    let one = |o: &Object| match o {
        Object::Ref(Ref::Named(id, gen)) if aml.ns.live(*id, *gen) => Dep::On(*id),
        Object::String(s) => {
            let at = scope.and_then(|sc| resolve_text(&aml.ns, sc, &s.borrow()));
            at.map_or(Dep::Dangling, Dep::On)
        }
        _ => Dep::Dangling,
    };
    p.iter().take(MAX).map(one).collect()
}
