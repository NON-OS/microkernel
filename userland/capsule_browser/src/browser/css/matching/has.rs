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

use crate::browser::css::selector::{Comb, Selector};

use super::cx::Cx;
use super::selector::test;
use super::tree_scan::{push_children, push_later};

/* :has() at element e: some element matches a relative argument anchored
 * at e. Each argument ends in a step whose compound only e satisfies, so
 * the ordinary matcher checks the relation; this only chooses where to
 * look. */
pub(super) fn has(cx: &Cx, e: usize, list: &[Selector]) -> bool {
    let saved = cx.anchor.replace(e);
    let hit = list.iter().any(|rel| scan(cx, e, rel));
    cx.anchor.set(saved);
    hit
}

/* Candidates are e's descendants when the argument opens with a child or
 * descendant combinator (only its children for a lone `> x`), and e's later
 * siblings when it opens with a sibling combinator (only the next one for a
 * lone `+ x`), with their subtrees when a child or descendant combinator
 * follows. Each candidate visited costs a step. */
fn scan(cx: &Cx, e: usize, rel: &Selector) -> bool {
    let Some((anchor, inner)) = rel.ancestors.split_last() else {
        return false;
    };
    let lone = inner.is_empty();
    let descend = match anchor.comb {
        Comb::Child => !lone,
        Comb::Descendant => true,
        _ => inner.iter().any(|s| !s.comb.is_sibling()),
    };
    let mut stack: Vec<usize> = Vec::new();
    if anchor.comb.is_sibling() {
        push_later(cx, e, anchor.comb == Comb::NextSibling && lone, &mut stack);
    } else {
        push_children(cx, e, &mut stack);
    }
    while let Some(d) = stack.pop() {
        if !cx.tick() {
            return false;
        }
        if test(cx, d, rel) {
            return true;
        }
        if descend {
            push_children(cx, d, &mut stack);
        }
    }
    false
}
