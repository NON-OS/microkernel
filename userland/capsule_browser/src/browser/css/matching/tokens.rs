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

use crate::browser::css::selector::{id_key, is_space, name_hash};
use crate::browser::dom::node::NodeKind;
use crate::browser::dom::Dom;

/* One class name of an element: its key and where it sits in the element's
 * class value. */
#[derive(Clone, Copy)]
pub(super) struct Tok {
    pub key: u64,
    pub start: u32,
    pub end: u32,
}

/* Every element's class names, split on ASCII whitespace as HTML splits a
 * class attribute, hashed once; and its id key. Returns the tokens, the
 * index of each node's first token (one entry more than there are nodes),
 * and the id keys (0 without an id). */
pub(super) fn tokens(dom: &Dom) -> (Vec<Tok>, Vec<u32>, Vec<u64>) {
    let n = dom.nodes.len();
    let (mut toks, mut at, mut ids) =
        (Vec::new(), Vec::with_capacity(n + 1), Vec::with_capacity(n));
    for node in &dom.nodes {
        at.push(toks.len() as u32);
        if node.kind != NodeKind::Element {
            ids.push(0);
            continue;
        }
        ids.push(node.attr("id").map_or(0, |v| id_key(v.as_bytes())));
        let Some(value) = node.attr("class") else { continue };
        let b = value.as_bytes();
        let mut i = 0;
        while i < b.len() {
            if is_space(b[i]) {
                i += 1;
                continue;
            }
            let start = i;
            while i < b.len() && !is_space(b[i]) {
                i += 1;
            }
            let key = name_hash(b'.', &b[start..i]);
            toks.push(Tok { key, start: start as u32, end: i as u32 });
        }
    }
    at.push(toks.len() as u32);
    (toks, at, ids)
}
