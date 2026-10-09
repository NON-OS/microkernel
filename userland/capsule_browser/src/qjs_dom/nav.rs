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

use super::ffi::dom;

#[no_mangle]
pub unsafe extern "C" fn njs_dom_parent(host: *mut c_void, node: i32) -> i32 {
    if node <= 0 {
        return -1;
    }
    match dom(host).nodes.get(node as usize) {
        Some(n) => n.parent as i32,
        None => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn njs_dom_child_count(host: *mut c_void, node: i32) -> i32 {
    if node < 0 {
        return 0;
    }
    match dom(host).nodes.get(node as usize) {
        Some(n) => n.children.len() as i32,
        None => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn njs_dom_child_at(host: *mut c_void, node: i32, i: i32) -> i32 {
    if node < 0 || i < 0 {
        return -1;
    }
    match dom(host).nodes.get(node as usize) {
        Some(n) => n.children.get(i as usize).map(|&c| c as i32).unwrap_or(-1),
        None => -1,
    }
}
