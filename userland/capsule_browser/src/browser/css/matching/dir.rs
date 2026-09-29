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

use crate::browser::dom::node::NodeKind;

use super::cx::Cx;

/* Parent hops the direction lookup climbs; a script can build a loop. */
const MAX_HOPS: u32 = 512;

/* The element's directionality (HTML 3.2.6.4): the nearest dir="ltr" or
 * dir="rtl" at or above it; dir="auto" (and a bdi without dir) takes the
 * first strong character of its text; left to right otherwise. */
pub(super) fn is_rtl(cx: &Cx, id: usize) -> bool {
    let mut node = id;
    for _ in 0..MAX_HOPS {
        let Some(n) = cx.dom.nodes.get(node) else { return false };
        match n.attr("dir").map(str::to_ascii_lowercase).as_deref() {
            Some("rtl") => return true,
            Some("ltr") => return false,
            Some("auto") => return first_strong_rtl(cx, node),
            None if n.tag == "bdi" => return first_strong_rtl(cx, node),
            _ => {}
        }
        if node == 0 || n.parent == node {
            return false;
        }
        node = n.parent;
    }
    false
}

/* The first character with a strong direction in the element's text, in
 * tree order; each node visited costs a step. Hebrew, Arabic, Syriac,
 * Thaana, N'Ko and the Arabic presentation forms read right to left, any
 * other letter left to right, and text with no letter left to right. */
fn first_strong_rtl(cx: &Cx, id: usize) -> bool {
    let mut stack: Vec<usize> = Vec::from([id]);
    while let Some(i) = stack.pop() {
        let Some(n) = cx.dom.nodes.get(i) else { continue };
        if !cx.tick() {
            return false;
        }
        if n.kind == NodeKind::Text {
            if let Some(c) = n.text.chars().find(|c| c.is_alphabetic()) {
                let u = c as u32;
                return matches!(u, 0x0590..=0x07FF | 0x08A0..=0x08FF | 0xFB1D..=0xFDFF | 0xFE70..=0xFEFF);
            }
        }
        stack.extend(n.children.iter().rev().copied().filter(|&k| k != i));
    }
    false
}
