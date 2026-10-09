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

//! document.cookie, answered from the jar of the network the page came over.

use core::ffi::c_void;

use crate::browser::{cookie, url};
use crate::qjs_dom::{cdup, cstr, dom};

fn now() -> i64 {
    cookie::unix_secs(nonos_libc::mk_time_millis())
}

/// The cookies the page's script may read: its own address's, without the
/// HttpOnly ones. A page with no address reads nothing.
#[no_mangle]
pub unsafe extern "C" fn njs_dom_cookie_get(host: *mut c_void) -> *mut u8 {
    match url::parse(&dom(host).base) {
        Some(at) => cdup(&cookie::script_get(&at, now())),
        None => cdup(""),
    }
}

/// One `document.cookie = line`. The jar refuses what a script may not set.
#[no_mangle]
pub unsafe extern "C" fn njs_dom_cookie_set(host: *mut c_void, line: *const u8) {
    if let Some(at) = url::parse(&dom(host).base) {
        cookie::script_set(&at, &cstr(line), now());
    }
}
