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

//! A device's properties with the data-only subnodes the hierarchical data
//! extension hangs below it (`link`). A node is a package laid out as _DSD
//! is, with properties and children of its own. Bounded in depth and in
//! nodes; a node reached twice (a loop) is read the first time only.

use super::link::{links, target};
use super::set::{pairs, set_in};
use super::uuid::DEVICE_PROPERTIES_UUID;
use super::value::Property;
use crate::load::Aml;
use crate::namespace::Id;
use crate::object::Object;
use crate::platform::Eval;
use alloc::vec::Vec;

/// Nodes in one tree, the device's own included.
pub const MAX_NODES: usize = 32;
const MAX_LEVELS: u32 = 4;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Node {
    /// Empty for the device itself.
    pub name: Vec<u8>,
    pub properties: Vec<Property>,
    pub children: Vec<Node>,
}

struct Walk<'a, 'b> {
    aml: &'a mut Aml,
    eval: &'a mut Eval<'b>,
    seen: Vec<Id>,
}

impl Walk<'_, '_> {
    fn node(&mut self, at: Id, name: Vec<u8>, level: u32) -> Option<Node> {
        if self.seen.len() >= MAX_NODES || self.seen.contains(&at) {
            return None;
        }
        self.seen.push(at);
        let Ok(Object::Package(dsd)) = (self.eval)(self.aml, at, Vec::new()) else { return None };
        let dsd = dsd.borrow().clone();
        let set = set_in(&dsd, &DEVICE_PROPERTIES_UUID);
        let properties = set.map(|s| pairs(self.aml, &s)).unwrap_or_default();
        let mut children = Vec::new();
        for (n, t) in links(&dsd).into_iter().filter(|_| level < MAX_LEVELS) {
            if let Some(id) = target(self.aml, at, &t) {
                children.extend(self.node(id, n, level + 1));
            }
        }
        Some(Node { name, properties, children })
    }
}

/// The device's tree; an empty node when it has no _DSD or it fails.
pub fn tree(aml: &mut Aml, eval: &mut Eval<'_>, dev: Id) -> Node {
    let Some(dsd) = aml.ns.child(dev, *b"_DSD") else { return Node::default() };
    let mut w = Walk { aml, eval, seen: Vec::new() };
    w.node(dsd, Vec::new(), 0).unwrap_or_default()
}
