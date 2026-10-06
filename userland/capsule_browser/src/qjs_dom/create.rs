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

use crate::browser::dom::node::NodeKind;

use super::ffi::{cstr, dom};

#[no_mangle]
pub unsafe extern "C" fn njs_dom_create_element(host: *mut c_void, tag: *const u8) -> i32 {
    dom(host).create(NodeKind::Element, cstr(tag)).map(|i| i as i32).unwrap_or(-1)
}

#[no_mangle]
pub unsafe extern "C" fn njs_dom_append(host: *mut c_void, parent: i32, child: i32) {
    if parent >= 0 && child >= 0 {
        dom(host).attach(parent as usize, child as usize);
    }
}

#[no_mangle]
pub unsafe extern "C" fn njs_dom_remove_child(host: *mut c_void, _parent: i32, child: i32) {
    if child >= 0 {
        dom(host).detach(child as usize);
    }
}

#[no_mangle]
pub unsafe extern "C" fn njs_dom_create_text(host: *mut c_void, text: *const u8) -> i32 {
    let d = dom(host);
    match d.create(NodeKind::Text, String::new()) {
        Some(id) => {
            d.nodes[id].text = cstr(text);
            id as i32
        }
        None => -1,
    }
}

/// Framework diffing inserts relative to a reference sibling. A missing or
/// foreign reference degrades to a plain append, which attach() provides.
#[no_mangle]
pub unsafe extern "C" fn njs_dom_insert_before(
    host: *mut c_void,
    parent: i32,
    child: i32,
    before: i32,
) {
    if parent < 0 || child < 0 {
        return;
    }
    /*
     * The reference position is read after the node leaves the list: taken
     * first, it puts a node moved forwards one slot late, and a reordering
     * list drifts further out of order with every update. `place` also
     * unwraps a fragment rather than putting the holder in the page.
     */
    let before = if before < 0 { usize::MAX } else { before as usize };
    dom(host).place(parent as usize, child as usize, before);
}
