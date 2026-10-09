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

use core::ffi::c_void;

use crate::browser::dom::{parse_fragment, Dom};

use super::ffi::{cstr, dom};

const MAX_GRAFT_DEPTH: u32 = 64;

fn graft(dst: &mut Dom, src: &Dom, src_id: usize, dst_id: usize, depth: u32) {
    if depth > MAX_GRAFT_DEPTH {
        return;
    }
    let children = match src.nodes.get(src_id) {
        Some(n) => n.children.clone(),
        None => return,
    };
    for c in children {
        let (kind, tag, text, attrs, ns) = match src.nodes.get(c) {
            Some(n) => (n.kind, n.tag.clone(), n.text.clone(), n.attrs.clone(), n.ns),
            None => continue,
        };
        let Some(nid) = dst.push(dst_id, kind, tag) else {
            return;
        };
        dst.nodes[nid].text = text;
        dst.nodes[nid].attrs = attrs;
        dst.nodes[nid].ns = ns;
        graft(dst, src, c, nid, depth + 1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn njs_dom_set_inner_html(host: *mut c_void, node: i32, html: *const u8) {
    if node < 0 {
        return;
    }
    let d = dom(host);
    if node as usize >= d.nodes.len() {
        return;
    }
    /*
     * Parsed as the contents of the target, as innerHTML is, so a row goes
     * into a tbody as a row and nothing wraps it in html and body.
     */
    let context = d.nodes[node as usize].context_tag();
    let frag = parse_fragment(cstr(html).as_bytes(), &context);
    d.nodes[node as usize].children.clear();
    graft(d, &frag, 0, node as usize, 0);
}
