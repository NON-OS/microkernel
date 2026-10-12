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

//! Visiting the namespace depth first, each object before its children and
//! siblings in definition order. The visitor says whether to go below the
//! object; objects the operating system defined itself are passed over, as
//! they are not firmware's devices.

use crate::load::Aml;
use crate::namespace::{Id, Origin, ROOT};
use alloc::vec::Vec;

/// A namespace is far smaller; the bound stops a table loaded mid-walk from
/// making the walk endless.
const MAX_VISITS: usize = 1 << 20;

fn push_children(aml: &Aml, stack: &mut Vec<Id>, id: Id) {
    let at = stack.len();
    for c in aml.ns.children(id) {
        stack.insert(at, c);
    }
}

pub(super) fn walk(aml: &mut Aml, visit: &mut dyn FnMut(&mut Aml, Id) -> bool) {
    let mut stack = Vec::new();
    push_children(aml, &mut stack, ROOT);
    let mut visits = 0;
    while let Some(id) = stack.pop() {
        visits += 1;
        if visits > MAX_VISITS {
            return;
        }
        let Some(e) = aml.ns.get(id) else { continue };
        let predefined = matches!(e.origin, Origin::Predefined(_));
        if predefined || visit(aml, id) {
            push_children(aml, &mut stack, id);
        }
    }
}
