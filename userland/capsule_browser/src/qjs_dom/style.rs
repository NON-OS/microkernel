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

use super::ffi::{cstr, dom};

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
    /* style.cssText replaces the whole inline style rather than appending. */
    let key = cstr(prop);
    if key == "cssText" {
        d.set_attr(node as usize, "style", cstr(val));
        return;
    }
    let mut style = d
        .nodes
        .get(node as usize)
        .and_then(|n| n.attrs.iter().find(|(k, _)| k == "style").map(|(_, v)| v.clone()))
        .unwrap_or_default();
    style.push_str(&kebab(&key));
    style.push(':');
    style.push_str(&cstr(val));
    style.push(';');
    d.set_attr(node as usize, "style", style);
}

/// camelCase CSS keys as frameworks emit them (borderRadius) to the kebab-case
/// the stylesheet parser expects (border-radius).
fn kebab(s: &str) -> String {
    let mut out = String::new();
    for c in s.chars() {
        if c.is_ascii_uppercase() {
            out.push('-');
            out.push(c.to_ascii_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}
