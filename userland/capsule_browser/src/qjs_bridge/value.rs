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

//! A script's `el.value`, read and written as a form control holds it.
//!
//! It was the value attribute both ways, so a select read as empty whatever
//! it showed, a textarea the page filled in read as empty, and setting a
//! select's value chose nothing. It now goes through the same reading a
//! submit makes (event::field_value).

use core::ffi::c_void;

use crate::browser::event::{control_value, set_control_value};
use crate::qjs_dom::{cdup, cstr, dom};

/// The control's value as a C string the caller frees.
#[no_mangle]
pub unsafe extern "C" fn njs_dom_get_value(host: *mut c_void, node: i32) -> *mut u8 {
    if node < 0 {
        return cdup("");
    }
    cdup(&control_value(dom(host), node as usize))
}

/// Set the control's value (a select chooses the option sending it).
#[no_mangle]
pub unsafe extern "C" fn njs_dom_set_value(host: *mut c_void, node: i32, v: *const u8) {
    if node >= 0 {
        set_control_value(dom(host), node as usize, &cstr(v));
    }
}
