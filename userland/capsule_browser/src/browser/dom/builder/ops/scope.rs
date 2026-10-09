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

use super::counts::bucket;
use super::meta::{BUTTON_BOUND, DEFAULT_BOUND, LIST_BOUND, TABLE_BOUND};
use super::state::Builder;

/// The kinds of scope an element can be "in" (13.2.4.2).
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Scope {
    Default,
    ListItem,
    Button,
    Table,
}

impl Scope {
    fn bounds(self) -> u16 {
        match self {
            Scope::Default => DEFAULT_BOUND,
            Scope::ListItem => DEFAULT_BOUND | LIST_BOUND,
            Scope::Button => DEFAULT_BOUND | BUTTON_BOUND,
            Scope::Table => TABLE_BOUND,
        }
    }
}

impl Builder {
    /// Whether an HTML element named `name` is in `scope`: a walk down from
    /// the current node, skipped when nothing with that name is open.
    pub(in super::super) fn in_scope(&self, name: &str, scope: Scope) -> bool {
        if !self.maybe_open(name) {
            return false;
        }
        let want = bucket(name) as u16;
        self.scope_find(scope, |b, i| b.open_meta[i] & 0xFF == want && b.is(b.open[i], name))
    }

    /// Whether an HTML element with one of `names` is in the default scope.
    pub(in super::super) fn any_in_scope(&self, names: &[&str]) -> bool {
        let open = names.iter().any(|n| self.maybe_open(n));
        open && self.scope_find(Scope::Default, |b, i| b.is_any(b.open[i], names))
    }

    /// Whether this very element is in the default scope.
    pub(in super::super) fn node_in_scope(&self, target: usize) -> bool {
        self.on_stack(target) && self.scope_find(Scope::Default, |b, i| b.open[i] == target)
    }

    fn scope_find(&self, scope: Scope, hit: impl Fn(&Builder, usize) -> bool) -> bool {
        let bounds = scope.bounds();
        for i in (0..self.open.len()).rev() {
            if hit(self, i) {
                return true;
            }
            if self.open_meta[i] & bounds != 0 {
                return false;
            }
        }
        false
    }
}
