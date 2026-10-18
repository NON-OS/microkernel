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

//! The parser's view of the namespace: where it stands, and how many
//! arguments a name it meets takes.

use super::scope_path::joined;
use crate::grammar::NamePath;
use crate::namespace::{absolute, Id, Namespace, Seg};
use crate::parse::Scopes;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

/// Tracks the parser's scope as an absolute path, which need not exist in
/// the namespace yet: a table is parsed before its objects are added. Methods
/// the table declares as it goes are remembered beside the namespace, so a
/// load-time call to one defined earlier in the same table is read with its
/// arguments.
pub struct ParseScopes<'n> {
    pub(super) ns: &'n Namespace,
    path: Vec<Seg>,
    saved: Vec<Vec<Seg>>,
    pub(super) declared: BTreeMap<Vec<Seg>, u8>,
}

impl<'n> ParseScopes<'n> {
    /// Stands in `scope`: the root for a table, a method for its body.
    pub fn new(ns: &'n Namespace, scope: Id) -> Self {
        Self { ns, path: absolute(ns, scope), saved: Vec::new(), declared: BTreeMap::new() }
    }
}

impl Scopes for ParseScopes<'_> {
    fn enter(&mut self, bytes: &[u8], name: NamePath) {
        let next = joined(&self.path, bytes, name);
        self.saved.push(core::mem::replace(&mut self.path, next));
    }
    fn leave(&mut self) {
        if let Some(p) = self.saved.pop() {
            self.path = p;
        }
    }
    fn declare(&mut self, bytes: &[u8], name: NamePath, args: u8) {
        self.declared.insert(joined(&self.path, bytes, name), args);
    }
    /// Section 5.3's search rules for a lone NameSeg, the exact path for
    /// anything else.
    fn arity(&self, bytes: &[u8], name: NamePath) -> u8 {
        if name.root || name.up > 0 || name.count != 1 {
            return self.lookup(&joined(&self.path, bytes, name)).unwrap_or(0);
        }
        let seg = name.seg(bytes, 0);
        (0..=self.path.len())
            .rev()
            .find_map(|depth| self.lookup(&[&self.path[..depth], &[seg][..]].concat()))
            .unwrap_or(0)
    }
}
