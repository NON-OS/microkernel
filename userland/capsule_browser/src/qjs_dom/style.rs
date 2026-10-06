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

//! `el.style` from a script: one property of the node's style attribute,
//! read or replaced (css::parse::inline_style).

use core::ffi::c_void;

use super::ffi::{cdup, cstr, dom};
use crate::browser::css::parse::{style_get, style_set};
use crate::browser::dom::Dom;

/* The node's style attribute, or empty. */
fn inline(d: &Dom, node: usize) -> &str {
    d.nodes.get(node).and_then(|n| n.attr("style")).unwrap_or("")
}

#[no_mangle]
pub unsafe extern "C" fn njs_dom_set_style(
    host: *mut c_void,
    node: i32,
    prop: *const u8,
    val: *const u8,
) {
    if node < 0 {
        return;
    }
    let d = dom(host);
    /* style.cssText replaces the whole inline style. */
    let key = cstr(prop);
    if key == "cssText" {
        d.set_attr(node as usize, "style", cstr(val));
        return;
    }
    let style = style_set(inline(d, node as usize), &key, &cstr(val));
    if style.is_empty() {
        d.remove_attr(node as usize, "style");
    } else {
        d.set_attr(node as usize, "style", style);
    }
}

/// What the node's inline style gives `prop`, as a C string the caller
/// frees: empty when it gives nothing, which is what a script reads then.
/// `cssText` is the whole attribute.
#[no_mangle]
pub unsafe extern "C" fn njs_dom_get_style(
    host: *mut c_void,
    node: i32,
    prop: *const u8,
) -> *mut u8 {
    if node < 0 {
        return cdup("");
    }
    let d = dom(host);
    let key = cstr(prop);
    if key == "cssText" {
        return cdup(inline(d, node as usize));
    }
    cdup(&style_get(inline(d, node as usize), &key))
}
