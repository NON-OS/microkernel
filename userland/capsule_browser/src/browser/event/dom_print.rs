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

use crate::browser::dom::Dom;

const K: u64 = 0x9e37_79b9_7f4a_7c15;

/* A fingerprint of everything a style or a layout of the document reads:
 * each node's kind, parent, children, tag, attributes and text, and the
 * fetched CSS text. It stands in for a mutation counter the tree does not
 * keep: equal prints mean the same document to style. The second print
 * leaves out value attributes, which typing changes and which few
 * selectors read. Eight bytes are mixed per step. */
pub fn dom_print(dom: &Dom, page_css: &str) -> (u64, u64) {
    let (mut full, mut sans) = (mix(K, page_css.as_bytes()), mix(K, page_css.as_bytes()));
    for (id, n) in dom.nodes.iter().enumerate() {
        let mut d = step(id as u64, n.kind as u64 ^ ((n.parent as u64) << 8));
        d = mix(d, n.tag.as_bytes());
        d = mix(d, n.text.as_bytes());
        for &c in &n.children {
            d = step(d, c as u64);
        }
        let mut value = 0;
        for (k, v) in &n.attrs {
            if k == "value" {
                value = mix(value ^ K, v.as_bytes());
                continue;
            }
            d = mix(mix(d, k.as_bytes()), v.as_bytes());
        }
        sans = step(sans, d);
        full = step(step(full, d), value);
    }
    (full, sans)
}

fn step(h: u64, w: u64) -> u64 {
    let h = (h ^ w).wrapping_mul(K);
    h ^ (h >> 29)
}

/* `bytes` folded into `h` eight at a time, the tail and the length last,
 * so "ab" + "c" and "a" + "bc" differ. */
fn mix(mut h: u64, bytes: &[u8]) -> u64 {
    let mut words = bytes.chunks_exact(8);
    for w in &mut words {
        h = step(h, u64::from_le_bytes([w[0], w[1], w[2], w[3], w[4], w[5], w[6], w[7]]));
    }
    let mut tail = [0u8; 8];
    tail[..words.remainder().len()].copy_from_slice(words.remainder());
    step(step(h, u64::from_le_bytes(tail)), bytes.len() as u64)
}
