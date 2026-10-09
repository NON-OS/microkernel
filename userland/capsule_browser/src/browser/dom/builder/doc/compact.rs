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

use alloc::vec;
use alloc::vec::Vec;

use super::super::super::limits::{MAX_DEPTH, TRUNC_DEPTH};
use super::super::super::node::NodeKind;
use super::super::super::tree::Dom;

/// Renumber the arena in document order and drop what is not in the tree.
///
/// Misnested markup moves nodes after they were made, so code that walks
/// `nodes` front to back (scripts, style sheets) would see them out of order;
/// afterwards every parent precedes its children. Nodes left out of the tree
/// (a replaced body, the fragment root and context) are gone, and content
/// moved deeper than `MAX_DEPTH` is dropped and recorded in `truncated`.
pub fn compact(dom: &mut Dom) {
    let n = dom.nodes.len();
    let mut map = vec![u32::MAX; n];
    let mut next = 0u32;
    let mut stack: Vec<(usize, usize)> = vec![(0, 0)];
    while let Some((id, depth)) = stack.pop() {
        map[id] = next;
        next += 1;
        if depth >= MAX_DEPTH {
            let kept: Vec<usize> = dom.nodes[id]
                .children
                .iter()
                .copied()
                .filter(|&c| depth == MAX_DEPTH && dom.nodes[c].kind == NodeKind::Text)
                .collect();
            if kept.len() != dom.nodes[id].children.len() {
                dom.truncated |= TRUNC_DEPTH;
                dom.nodes[id].children = kept;
            }
        }
        for &c in dom.nodes[id].children.iter().rev() {
            stack.push((c, depth + 1));
        }
    }
    let kept = next as usize;
    for m in map.iter_mut().filter(|m| **m == u32::MAX) {
        *m = next;
        next += 1;
    }
    for node in dom.nodes.iter_mut() {
        node.parent = map[node.parent] as usize;
        for c in node.children.iter_mut() {
            *c = map[*c] as usize;
        }
    }
    /* Move each node to its new index by following the permutation's cycles. */
    for i in 0..n {
        while map[i] as usize != i {
            let to = map[i] as usize;
            dom.nodes.swap(i, to);
            map.swap(i, to);
        }
    }
    dom.nodes.truncate(kept);
}
