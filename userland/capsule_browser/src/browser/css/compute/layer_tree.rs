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
use alloc::vec::Vec;

use crate::browser::css::rule::Rule;

/* Layers kept apart. A layer first named past this merges into the last
 * top-level layer, the highest layered rank, so it still ranks after
 * every earlier layer and below unlayered rules, and the ranks stay far
 * below Rule::UNLAYERED. */
const MAX_LAYERS: usize = 16_384;

/* The layer at dotted `path`, created in order of first appearance. */
pub(super) fn node(tree: &mut Vec<(Rc<str>, Vec<usize>)>, path: &str) -> usize {
    let mut at = 0;
    for seg in path.split('.').map(str::trim) {
        let found = tree[at].1.iter().copied().find(|&c| &*tree[c].0 == seg);
        at = match found {
            Some(c) => c,
            None if tree.len() < MAX_LAYERS => {
                tree.push((Rc::from(seg), Vec::new()));
                let c = tree.len() - 1;
                tree[at].1.push(c);
                c
            }
            None => return tree[0].1.last().copied().unwrap_or(0),
        };
    }
    at
}

/* Rank the layers under the root in post order, sublayers first, with an
 * explicit stack: a dotted name may nest thousands deep. */
pub(super) fn post_order(tree: &[(Rc<str>, Vec<usize>)], ranks: &mut [u16]) {
    let (mut stack, mut next) = (Vec::from([(0usize, 0usize)]), 0u16);
    while let Some(top) = stack.last_mut() {
        let (at, i) = *top;
        if let Some(&c) = tree[at].1.get(i) {
            top.1 += 1;
            stack.push((c, 0));
            continue;
        }
        stack.pop();
        ranks[at] = next;
        next = next.saturating_add(1).min(Rule::UNLAYERED - 1);
    }
}
