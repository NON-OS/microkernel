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

//! Where the page came from, and what a relative address means from there.

use core::ffi::c_void;
use core::ptr;

use crate::browser::dom::script_scroll::Block;
use crate::browser::url;
use crate::qjs_dom::{cdup, cstr, dom};

/// The address the document was loaded from.
///
/// `location` reported `http://localhost/` no matter what had been fetched.
/// A page that reads its own path to decide what to show, which is every
/// page with a router in it, was told something that was never true.
#[no_mangle]
pub unsafe extern "C" fn njs_dom_base_url(host: *mut c_void) -> *mut u8 {
    cdup(&dom(host).base)
}

/// A relative address made absolute against the page.
///
/// `href` and `src` came back exactly as markup spelled them, so a script
/// reading a link got "/about" where it expected the whole address, and
/// anything comparing it against a real one disagreed.
#[no_mangle]
pub unsafe extern "C" fn njs_dom_resolve(host: *mut c_void, rel: *const u8) -> *mut u8 {
    let d = dom(host);
    let relative = cstr(rel);
    if relative.is_empty() {
        return ptr::null_mut();
    }
    // Without a base there is nothing to resolve against, and guessing one
    // would turn a relative address into a confident wrong one.
    match url::parse(&d.base) {
        Some(base) => cdup(&url::join(&base, &relative)),
        None => cdup(&relative),
    }
}

/// The reader's session history for `history.length`: with `which` 0 how
/// many entries it holds, with 1 which of them is shown.
#[no_mangle]
pub unsafe extern "C" fn njs_dom_history(host: *mut c_void, which: i32) -> i32 {
    let (n, at) = dom(host).history;
    (if which == 0 { n } else { at }).min(i32::MAX as u32) as i32
}

/// How far the page is scrolled down, in pixels.
#[no_mangle]
pub unsafe extern "C" fn njs_dom_scroll_y(host: *mut c_void) -> i32 {
    dom(host).scroll_y.min(i32::MAX as u32) as i32
}

/// A script's `scrollTo`, `scroll` or `scrollBy`: scroll the page to `y`
/// within what it can scroll, and answer where it is now. The window
/// follows when the script returns (event::scroll_by).
#[no_mangle]
pub unsafe extern "C" fn njs_dom_scroll_to(host: *mut c_void, y: i32) -> i32 {
    dom(host).script_scroll(y as i64).min(i32::MAX as u32) as i32
}

/// A script's `el.scrollIntoView`, with `block` as `Block::from_code`
/// reads it. A node that was not laid out moves nothing.
#[no_mangle]
pub unsafe extern "C" fn njs_dom_scroll_into_view(host: *mut c_void, node: i32, block: i32) -> i32 {
    let d = dom(host);
    let want = match node {
        n if n < 0 => None,
        n => d.into_view_y(n as usize, Block::from_code(block)),
    };
    match want {
        Some(y) => d.script_scroll(y),
        None => d.scroll_y,
    }
    .min(i32::MAX as u32) as i32
}

/// One number from a node's laid-out box, as `getBoundingClientRect` and the
/// offset properties read it.
#[no_mangle]
pub unsafe extern "C" fn njs_dom_box(host: *mut c_void, node: i32, which: i32) -> i32 {
    if node < 0 || which < 0 {
        return 0;
    }
    dom(host).box_of(node as usize, which as usize)
}
