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
use core::ptr;

use super::ffi::{cdup, cstr, dom};

#[no_mangle]
pub unsafe extern "C" fn njs_dom_set_attr(
    host: *mut c_void,
    node: i32,
    k: *const u8,
    v: *const u8,
) {
    if node >= 0 {
        dom(host).set_attr(node as usize, &cstr(k), cstr(v));
    }
}

#[no_mangle]
pub unsafe extern "C" fn njs_dom_get_attr(host: *mut c_void, node: i32, k: *const u8) -> *mut u8 {
    if node < 0 {
        return ptr::null_mut();
    }
    let key = cstr(k);
    match dom(host).nodes.get(node as usize) {
        Some(n) => match n.attrs.iter().find(|(a, _)| *a == key) {
            Some((_, val)) => cdup(val),
            None => ptr::null_mut(),
        },
        None => ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn njs_dom_remove_attr(host: *mut c_void, node: i32, k: *const u8) {
    if node >= 0 {
        let key = cstr(k);
        dom(host).remove_attr(node as usize, &key);
    }
}
