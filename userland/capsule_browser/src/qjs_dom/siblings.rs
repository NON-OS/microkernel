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

use crate::browser::dom::node::NodeKind;

use super::ffi::dom;

#[no_mangle]
pub unsafe extern "C" fn njs_dom_next_sibling(host: *mut c_void, node: i32) -> i32 {
    if node <= 0 {
        return -1;
    }
    let d = dom(host);
    let Some(n) = d.nodes.get(node as usize) else {
        return -1;
    };
    let Some(p) = d.nodes.get(n.parent) else {
        return -1;
    };
    match p.children.iter().position(|&c| c == node as usize) {
        Some(i) => p.children.get(i + 1).map(|&c| c as i32).unwrap_or(-1),
        None => -1,
    }
}

/// DOM nodeType numbering: 1 element, 3 text, 9 document.
#[no_mangle]
pub unsafe extern "C" fn njs_dom_node_kind(host: *mut c_void, node: i32) -> i32 {
    if node < 0 {
        return 0;
    }
    match dom(host).nodes.get(node as usize).map(|n| n.kind) {
        Some(NodeKind::Element) => 1,
        Some(NodeKind::Text) => 3,
        Some(NodeKind::Document) => 9,
        None => 0,
    }
}
