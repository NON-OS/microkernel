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

//! Climbing to where a path starts, and through aliases.

use super::{Id, Kind, Namespace, ROOT};

/// An alias may name another alias; a chain this long is a table looping
/// aliases into each other, not a namespace.
const MAX_ALIAS_HOPS: usize = 8;

impl Namespace {
    /// The root, or `scope` climbed `up` times; climbing past the root names
    /// nothing.
    pub(super) fn start(&self, scope: Id, root: bool, up: u8) -> Option<Id> {
        if root {
            return Some(ROOT);
        }
        (0..up).try_fold(scope, |at, _| (at != ROOT).then(|| self.get(at).map(|e| e.parent))?)
    }

    /// The object an alias names, through at most `MAX_ALIAS_HOPS` aliases.
    pub fn unalias(&self, mut id: Id) -> Option<Id> {
        for _ in 0..MAX_ALIAS_HOPS {
            match self.get(id)?.kind {
                Kind::Alias(target) => id = target,
                _ => return Some(id),
            }
        }
        None
    }
}
