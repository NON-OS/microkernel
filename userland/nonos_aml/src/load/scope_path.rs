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

//! Paths as the parser tracks them, and what lives at one.

use super::ParseScopes;
use crate::grammar::NamePath;
use crate::namespace::{Kind, Seg, ROOT};
use alloc::vec::Vec;

impl ParseScopes<'_> {
    /// The arity of the object at `path`: Some(0) for one that is not a
    /// method, None for none at all.
    pub(super) fn lookup(&self, path: &[Seg]) -> Option<u8> {
        if let Some(args) = self.declared.get(path) {
            return Some(*args);
        }
        let id = path.iter().try_fold(ROOT, |at, s| self.ns.unalias(self.ns.child(at, *s)?))?;
        match self.ns.get(id)?.kind {
            Kind::Method(m) => Some(m.args()),
            _ => Some(0),
        }
    }
}

/// `name` applied to the absolute `path`: from the root, or up and then down.
pub(super) fn joined(path: &[Seg], bytes: &[u8], name: NamePath) -> Vec<Seg> {
    let mut p = if name.root { Vec::new() } else { path.to_vec() };
    p.truncate(p.len().saturating_sub(name.up as usize));
    p.extend((0..name.count).map(|i| name.seg(bytes, i)));
    p
}
