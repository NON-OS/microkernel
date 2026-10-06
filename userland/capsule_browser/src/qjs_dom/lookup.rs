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

use crate::browser::css;

use super::ffi::{cdup, cstr, dom};

#[no_mangle]
pub unsafe extern "C" fn njs_dom_get_tag(host: *mut c_void, node: i32) -> *mut u8 {
    match dom(host).nodes.get(node as usize) {
        Some(n) => cdup(&n.tag),
        None => ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn njs_dom_get_by_id(host: *mut c_void, id: *const u8) -> i32 {
    let want = cstr(id);
    for (i, n) in dom(host).nodes.iter().enumerate() {
        if n.attrs.iter().any(|(k, v)| k == "id" && *v == want) {
            return i as i32;
        }
    }
    -1
}

#[no_mangle]
pub unsafe extern "C" fn njs_dom_query(host: *mut c_void, sel: *const u8) -> i32 {
    css::select(dom(host), &cstr(sel), 1).first().map(|i| *i as i32).unwrap_or(-1)
}

#[no_mangle]
pub unsafe extern "C" fn njs_dom_query_all(
    host: *mut c_void,
    sel: *const u8,
    out: *mut i32,
    max: i32,
) -> i32 {
    let cap = max.max(0) as usize;
    let hits = css::select(dom(host), &cstr(sel), cap);
    let n = hits.len().min(cap);
    for (i, &id) in hits.iter().take(n).enumerate() {
        *out.add(i) = id as i32;
    }
    n as i32
}

#[no_mangle]
pub unsafe extern "C" fn njs_dom_body(host: *mut c_void) -> i32 {
    for (i, n) in dom(host).nodes.iter().enumerate() {
        if n.tag == "body" {
            return i as i32;
        }
    }
    0
}
