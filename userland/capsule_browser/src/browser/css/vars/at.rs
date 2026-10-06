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

use alloc::rc::Rc;

use crate::browser::css::rule_index::key::hash_name;

use super::props::Props;
use super::scope::VarScope;
use super::subst::Lookup;

/* var() on element `id`: the nearest declaration in its scope. A
 * property registered with inherits: false counts only when the element
 * itself declared it, and a registered property with no value takes its
 * initial value. */
pub(in crate::browser::css) struct At<'a> {
    pub scope: &'a VarScope,
    pub props: &'a Props,
    pub id: usize,
}

impl Lookup for At<'_> {
    fn var(&mut self, name: &str) -> Option<Rc<str>> {
        let h = hash_name(name, true);
        let reg = self.props.get(h, name);
        let found = self.scope.find(h, name);
        match (found, reg) {
            (Some(v), None) => v.value.clone(),
            (Some(v), Some(p)) if p.inherits || v.owner == self.id => {
                v.value.clone().or_else(|| p.initial.clone())
            }
            (_, Some(p)) => p.initial.clone(),
            (None, None) => None,
        }
    }
}
