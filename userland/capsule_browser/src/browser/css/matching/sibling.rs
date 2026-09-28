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

use alloc::collections::BTreeMap;
use alloc::vec;
use alloc::vec::Vec;

use crate::browser::dom::node::NodeKind;
use crate::browser::dom::Dom;

/// Where each element sits among its element siblings.
///
/// A positional pseudo-class asks this once per element it is tested on, and
/// answering it by walking the parent's children made every such rule cost
/// the square of the sibling count: a 50,000 item list under one nth-child
/// rule held the cascade for 12.8 s on the host. A pass that runs over every
/// element builds the table once, in one walk of the tree; a check about a
/// single node, which runs per event, keeps the walk, since the table would
/// cost more than the one answer.
pub struct Siblings {
    table: Option<Vec<[i32; 4]>>,
}

impl Siblings {
    /// Answer each question by walking the parent's children.
    pub const fn walk() -> Self {
        Siblings { table: None }
    }

    /// Answer from a table built now, in one pass over the tree.
    pub fn table(dom: &Dom) -> Self {
        let mut t = vec![[0i32; 4]; dom.nodes.len()];
        let mut of_type: BTreeMap<&str, i32> = BTreeMap::new();
        for (p, parent) in dom.nodes.iter().enumerate() {
            of_type.clear();
            let mut count = 0;
            for &ch in &parent.children {
                let Some(c) = dom.nodes.get(ch) else { continue };
                if c.kind != NodeKind::Element {
                    continue;
                }
                count += 1;
                let ty = of_type.entry(c.tag.as_str()).or_insert(0);
                *ty += 1;
                // The walk reads a node's own parent's list, so a node a
                // script left listed under some other parent has no place.
                if c.parent == p {
                    t[ch][0] = count;
                    t[ch][2] = *ty;
                }
            }
            for &ch in &parent.children {
                let Some(c) = dom.nodes.get(ch) else { continue };
                if c.kind != NodeKind::Element || c.parent != p {
                    continue;
                }
                t[ch][1] = count;
                t[ch][3] = of_type.get(c.tag.as_str()).copied().unwrap_or(0);
            }
        }
        Siblings { table: Some(t) }
    }

    // 1-based position of an element among its element siblings, overall and
    // among those sharing its tag: (pos, count, pos_of_type, count_of_type).
    pub(super) fn position(&self, dom: &Dom, id: usize) -> Option<(i32, i32, i32, i32)> {
        match &self.table {
            Some(t) => {
                let [pos, count, pos_ty, count_ty] = *t.get(id)?;
                if pos == 0 {
                    return None;
                }
                Some((pos, count, pos_ty, count_ty))
            }
            None => element_position(dom, id),
        }
    }
}

fn element_position(dom: &Dom, id: usize) -> Option<(i32, i32, i32, i32)> {
    let node = dom.nodes.get(id)?;
    let parent = dom.nodes.get(node.parent)?;
    let mut pos = 0;
    let mut count = 0;
    let mut pos_ty = 0;
    let mut count_ty = 0;
    for &ch in &parent.children {
        let Some(c) = dom.nodes.get(ch) else { continue };
        if c.kind != NodeKind::Element {
            continue;
        }
        count += 1;
        let same_tag = c.tag == node.tag;
        if same_tag {
            count_ty += 1;
        }
        if ch == id {
            pos = count;
            pos_ty = count_ty;
        }
    }
    if pos == 0 {
        return None;
    }
    Some((pos, count, pos_ty, count_ty))
}
