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

use crate::browser::css::selector::{bloom_bits, name_hash};
use crate::browser::dom::node::NodeKind;
use crate::browser::dom::Dom;

use super::tokens::Tok;

/* For every node, a 128-bit filter holding two bits for the tag, id and
 * each class of every element above it, built in one walk down from the
 * document root. The walk follows a child only where the child names the
 * parent back, which is the chain the matcher climbs, so it can never loop.
 * A node the walk does not reach (detached, or listed under a parent that
 * is not its own) keeps every bit set, which rejects nothing. */
pub(super) fn ancestor_bloom(
    dom: &Dom,
    toks: &[Tok],
    tok_at: &[u32],
    ids: &[u64],
) -> Vec<[u64; 2]> {
    let n = dom.nodes.len();
    let mut out = vec![[u64::MAX; 2]; n];
    if n == 0 {
        return out;
    }
    out[0] = [0, 0];
    let mut seen = vec![false; n];
    let mut stack: Vec<usize> = vec![0];
    while let Some(p) = stack.pop() {
        let node = &dom.nodes[p];
        let mut mine = out[p];
        if node.kind == NodeKind::Element {
            let classes = &toks[tok_at[p] as usize..tok_at[p + 1] as usize];
            let tag = name_hash(b't', node.tag.as_bytes());
            let id = (ids[p] != 0).then_some(ids[p]);
            for key in classes.iter().map(|t| t.key).chain([tag]).chain(id) {
                let b = bloom_bits(key);
                mine = [mine[0] | b[0], mine[1] | b[1]];
            }
        }
        for &ch in &node.children {
            if ch != 0 && ch < n && dom.nodes[ch].parent == p && !seen[ch] {
                seen[ch] = true;
                out[ch] = mine;
                stack.push(ch);
            }
        }
    }
    out
}
