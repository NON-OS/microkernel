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

//! How the platform layer runs AML: through the caller's evaluator, so the
//! capsule supplies its Machine and host and the proofs their own.

use crate::exec::Fault;
use crate::load::Aml;
use crate::namespace::{Id, Seg};
use crate::object::Object;
use alloc::vec::Vec;

/// Evaluate the object at `id` with these arguments.
pub type Eval<'a> = dyn FnMut(&mut Aml, Id, Vec<Object>) -> Result<Object, Fault> + 'a;

/// The integer an object holds, if it is one.
pub fn integer(o: &Object) -> Option<u64> {
    match o {
        Object::Integer(v) => Some(*v),
        _ => None,
    }
}

/// Evaluate `parent`'s child named `seg`: None when there is no such child.
pub fn child(
    aml: &mut Aml,
    eval: &mut Eval<'_>,
    parent: Id,
    seg: Seg,
    args: Vec<Object>,
) -> Option<Result<Object, Fault>> {
    let id = aml.ns.child(parent, seg)?;
    Some(eval(aml, id, args))
}
