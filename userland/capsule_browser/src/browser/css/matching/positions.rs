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

/* Every element's [pos, count, pos_of_type, count_of_type] among its
 * element siblings and its previous element sibling (u32::MAX for none),
 * in one pass over every child list. Tags are numbered once for the whole
 * tree, and each parent's per-tag counts are reset lazily by stamping them
 * with the parent, so the pass stays linear however many distinct tags a
 * list holds. A node a script left listed under some other parent than its
 * own is counted where it is listed but given no place. */
pub(super) fn positions(dom: &Dom) -> (Vec<[i32; 4]>, Vec<u32>) {
    let n = dom.nodes.len();
    let (mut pos, mut prev) = (vec![[0i32; 4]; n], vec![u32::MAX; n]);
    let mut tag_no: BTreeMap<&str, usize> = BTreeMap::new();
    let (mut counts, mut stamp): (Vec<i32>, Vec<usize>) = (Vec::new(), Vec::new());
    let mut kids: Vec<(usize, usize)> = Vec::new();
    for (p, parent) in dom.nodes.iter().enumerate() {
        kids.clear();
        let (mut count, mut last) = (0, u32::MAX);
        for &ch in &parent.children {
            let Some(c) = dom.nodes.get(ch).filter(|c| c.kind == NodeKind::Element) else {
                continue;
            };
            let fresh = tag_no.len();
            let t = *tag_no.entry(c.tag.as_str()).or_insert(fresh);
            if t == counts.len() {
                counts.push(0);
                stamp.push(usize::MAX);
            }
            if stamp[t] != p {
                (stamp[t], counts[t]) = (p, 0);
            }
            count += 1;
            counts[t] += 1;
            if c.parent == p {
                (pos[ch][0], pos[ch][2], prev[ch]) = (count, counts[t], last);
                kids.push((ch, t));
            }
            last = ch as u32;
        }
        for &(ch, t) in &kids {
            (pos[ch][1], pos[ch][3]) = (count, counts[t]);
        }
    }
    (pos, prev)
}
