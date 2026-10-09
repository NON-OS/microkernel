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

use crate::browser::dom::node::NodeKind;
use crate::browser::dom::Dom;

use super::table::Table;

/// Where each element sits among its siblings, and what its ancestors are.
///
/// A pass that matches every element builds the table once, in one walk of
/// the tree: sibling positions (a 50,000 item list under one nth-child rule
/// otherwise costs the square of its length), previous element siblings,
/// each element's class names split and hashed, and a filter of every
/// ancestor's tag, id and classes that rejects most descendant selectors
/// without walking up. A check about a single node, which runs per event,
/// walks instead, since the table would cost more than the one answer.
pub struct Siblings {
    table: Option<Table>,
}

impl Siblings {
    /// Answer each question by walking the tree.
    pub const fn walk() -> Self {
        Siblings { table: None }
    }

    /// Answer from a table built now, in one pass over the tree.
    pub fn table(dom: &Dom) -> Self {
        Siblings { table: Some(Table::build(dom)) }
    }

    pub(super) fn tab(&self) -> Option<&Table> {
        self.table.as_ref()
    }

    /* 1-based position of an element among its element siblings, overall
     * and among those sharing its tag: (pos, count, pos_of_type,
     * count_of_type). */
    pub(super) fn position(&self, dom: &Dom, id: usize) -> Option<(i32, i32, i32, i32)> {
        match &self.table {
            Some(t) => t.position(id),
            None => walk_position(dom, id),
        }
    }
}

fn walk_position(dom: &Dom, id: usize) -> Option<(i32, i32, i32, i32)> {
    let node = dom.nodes.get(id)?;
    let (mut pos, mut count, mut pos_ty, mut count_ty) = (0, 0, 0, 0);
    for &ch in &dom.nodes.get(node.parent)?.children {
        let Some(c) = dom.nodes.get(ch).filter(|c| c.kind == NodeKind::Element) else { continue };
        count += 1;
        count_ty += (c.tag == node.tag) as i32;
        if ch == id {
            (pos, pos_ty) = (count, count_ty);
        }
    }
    (pos != 0).then_some((pos, count, pos_ty, count_ty))
}
