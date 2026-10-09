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

use alloc::vec::Vec;

use super::cx::Cx;

/* The element children of `p` that name it as their parent, pushed so they
 * pop in tree order. */
pub(super) fn push_children(cx: &Cx, p: usize, stack: &mut Vec<usize>) {
    let Some(node) = cx.dom.nodes.get(p) else { return };
    let kids = node.children.iter().rev().copied();
    stack.extend(kids.filter(|&k| cx.element(k).is_some_and(|c| c.parent == p)));
}

/* The first element under `root`, in tree order, that `f` accepts. Each
 * visit costs a step; a walk out of steps finds nothing, and the match it
 * serves answers no match. */
pub(super) fn find_under(cx: &Cx, root: usize, mut f: impl FnMut(usize) -> bool) -> Option<usize> {
    let mut stack: Vec<usize> = Vec::new();
    push_children(cx, root, &mut stack);
    while let Some(d) = stack.pop() {
        if !cx.tick() {
            return None;
        }
        if f(d) {
            return Some(d);
        }
        push_children(cx, d, &mut stack);
    }
    None
}

/* The element siblings after `e`, or only the first of them, pushed so
 * they pop in tree order. */
pub(super) fn push_later(cx: &Cx, e: usize, only_next: bool, stack: &mut Vec<usize>) {
    let parent = cx.dom.nodes.get(e).and_then(|n| cx.dom.nodes.get(n.parent));
    let Some(kids) = parent.map(|p| &p.children) else { return };
    let at = kids.iter().position(|&k| k == e).map_or(kids.len(), |i| i + 1);
    let mut later = kids[at..].iter().copied().filter(|&k| cx.element(k).is_some());
    if only_next {
        stack.extend(later.next());
    } else {
        stack.extend(later.rev());
    }
}
