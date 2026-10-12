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

//! Every object the namespace holds that `keep` accepts, in definition
//! order, without evaluating anything: for finding thermal zones and
//! backlit outputs by what they are and what they define.

use crate::load::Aml;
use crate::namespace::{Id, ROOT};
use alloc::vec::Vec;

const MAX_VISITS: usize = 1 << 20;

fn push_children(aml: &Aml, stack: &mut Vec<Id>, id: Id) {
    let at = stack.len();
    for c in aml.ns.children(id) {
        stack.insert(at, c);
    }
}

pub fn scan(aml: &Aml, keep: &dyn Fn(&Aml, Id) -> bool) -> Vec<Id> {
    let (mut out, mut stack) = (Vec::new(), Vec::new());
    push_children(aml, &mut stack, ROOT);
    for _ in 0..MAX_VISITS {
        let Some(id) = stack.pop() else { break };
        if keep(aml, id) {
            out.push(id);
        }
        if aml.ns.get(id).is_some_and(|e| e.kind.is_container()) {
            push_children(aml, &mut stack, id);
        }
    }
    out
}
