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

use alloc::string::String;
use core::ffi::c_void;
use core::ptr;

use crate::browser::dom::node::NodeKind;
use crate::browser::dom::Dom;

use super::ffi::{cdup, cstr, dom};

fn collect_text(d: &Dom, node: usize, out: &mut String) {
    if let Some(n) = d.nodes.get(node) {
        out.push_str(&n.text);
        for &c in &n.children {
            collect_text(d, c, out);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn njs_dom_set_text(host: *mut c_void, node: i32, text: *const u8) {
    if node < 0 {
        return;
    }
    let d = dom(host);
    /*
     * `node` is the JS-visible __node index, fully attacker controlled, so it
     * is bounds-checked before indexing; a hostile page setting a wild index
     * would otherwise abort the capsule (panic = abort in release).
     */
    if node as usize >= d.nodes.len() {
        return;
    }
    /*
     * A text node's data is its own text; frameworks update it in place.
     * Elements replace their subtree with one fresh text child.
     */
    if d.nodes[node as usize].kind == NodeKind::Text {
        d.nodes[node as usize].text = cstr(text);
        return;
    }
    d.nodes[node as usize].children.clear();
    if let Some(tid) = d.create(NodeKind::Text, String::new()) {
        d.nodes[tid].text = cstr(text);
        d.attach(node as usize, tid);
    }
}

#[no_mangle]
pub unsafe extern "C" fn njs_dom_get_text(host: *mut c_void, node: i32) -> *mut u8 {
    if node < 0 {
        return ptr::null_mut();
    }
    let mut s = String::new();
    collect_text(dom(host), node as usize, &mut s);
    cdup(&s)
}
