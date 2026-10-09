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

//! A script's `getComputedStyle(el).getPropertyValue(prop)`.

use core::ffi::c_void;

use crate::browser::css::parse::{css_name, style_get};
use crate::qjs_dom::{cdup, cstr, dom};

/// One property of the node's computed style, as a C string the caller
/// frees: the cascade's own value for the properties the layout keeps
/// (dom::style_facts), else what the node's style attribute declares,
/// else empty. `prop` may be in CSS or script spelling.
#[no_mangle]
pub unsafe extern "C" fn njs_dom_computed(
    host: *mut c_void,
    node: i32,
    prop: *const u8,
) -> *mut u8 {
    if node < 0 {
        return cdup("");
    }
    let d = dom(host);
    /* A custom property keeps its case; every other name has none. */
    let name = css_name(&cstr(prop));
    let name = if name.starts_with("--") { name } else { name.to_ascii_lowercase() };
    if let Some(v) = d.computed_value(node as usize, &name) {
        return cdup(&v);
    }
    let inline = d.nodes.get(node as usize).and_then(|n| n.attr("style")).unwrap_or("");
    cdup(&style_get(inline, &name))
}
