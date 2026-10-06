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
use core::slice;

use crate::browser::dom::Dom;

extern "C" {
    fn malloc(n: usize) -> *mut u8;
}

pub(crate) unsafe fn dom<'a>(host: *mut c_void) -> &'a mut Dom {
    &mut *(host as *mut Dom)
}

pub(crate) unsafe fn cstr(p: *const u8) -> String {
    if p.is_null() {
        return String::new();
    }
    let mut n = 0;
    while *p.add(n) != 0 {
        n += 1;
    }
    String::from_utf8_lossy(slice::from_raw_parts(p, n)).into_owned()
}

pub(crate) unsafe fn cdup(s: &str) -> *mut u8 {
    let b = s.as_bytes();
    let p = malloc(b.len() + 1);
    if p.is_null() {
        return p;
    }
    ptr::copy_nonoverlapping(b.as_ptr(), p, b.len());
    *p.add(b.len()) = 0;
    p
}
