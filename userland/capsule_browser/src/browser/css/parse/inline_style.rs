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

//! A script's `el.style`: one property of a style attribute read or written.
//!
//! Writing used to append `name:value;` to the attribute on every set and
//! reading answered nothing at all. So the menu toggle on most sites,
//! `s.display = s.display === 'none' ? 'block' : 'none'`, read undefined
//! every time and could close a menu but never open it again, and code that
//! moves an element each frame grew its style attribute without end. A set
//! now replaces the property's declaration, an empty value removes it, and
//! a read answers what the attribute holds, as CSSStyleDeclaration does.

use alloc::string::{String, ToString};

use super::parse_into::scan::split_top;

/// The CSS name of a `style` property as a script spells it: `fontSize`
/// is `font-size`, `cssFloat` is `float`, `webkitTransform` and
/// `WebkitTransform` are `-webkit-transform`. A name already in CSS form,
/// as setProperty is given, and a custom property are kept as they are.
pub fn css_name(prop: &str) -> String {
    if prop.starts_with("--") {
        return prop.to_string();
    }
    if prop == "cssFloat" {
        return String::from("float");
    }
    let vendor = ["webkit", "moz", "ms", "o"].iter().any(|v| {
        let cap = prop.get(..v.len()).is_some_and(|h| h.eq_ignore_ascii_case(v));
        cap && prop[v.len()..].starts_with(|c: char| c.is_ascii_uppercase())
    });
    let mut out = String::with_capacity(prop.len() + 4);
    if vendor {
        out.push('-');
    }
    for c in prop.chars() {
        if c.is_ascii_uppercase() {
            if !out.is_empty() && !out.ends_with('-') {
                out.push('-');
            }
            out.push(c.to_ascii_lowercase());
        } else {
            out.push(c.to_ascii_lowercase());
        }
    }
    out
}

/* The name and value of one "name: value" item, both trimmed. */
fn item(decl: &str) -> Option<(&str, &str)> {
    let (name, value) = decl.split_once(':')?;
    Some((name.trim(), value.trim()))
}

/* Whether `name` as written in the attribute is `want`. Custom properties
 * are case-sensitive; every other name is not. */
fn same(name: &str, want: &str) -> bool {
    if want.starts_with("--") {
        name == want
    } else {
        name.eq_ignore_ascii_case(want)
    }
}

/// The value the inline style `decls` gives property `prop` (script or CSS
/// spelling), without `!important`, or empty when it gives none. The last
/// declaration wins, as it does when the page is styled.
pub fn style_get(decls: &str, prop: &str) -> String {
    let want = css_name(prop);
    let mut found = "";
    for (name, value) in split_top(decls, b';').filter_map(item) {
        if same(name, &want) {
            found = value;
        }
    }
    let low = found.to_ascii_lowercase();
    match low.strip_suffix("important").map(str::trim_end) {
        Some(rest) if rest.ends_with('!') => found[..rest.len() - 1].trim_end().to_string(),
        _ => found.to_string(),
    }
}

/// The inline style `decls` with property `prop` set to `value`: every
/// declaration of it taken out, and one with the new value put last unless
/// the value is empty, which is how a script removes a property. The other
/// declarations keep their order and their text.
pub fn style_set(decls: &str, prop: &str, value: &str) -> String {
    let want = css_name(prop);
    let mut out = String::with_capacity(decls.len() + want.len() + value.len() + 2);
    for part in split_top(decls, b';').filter(|p| !p.is_empty()) {
        if item(part).is_some_and(|(name, _)| same(name, &want)) {
            continue;
        }
        out.push_str(part);
        out.push_str("; ");
    }
    let value = value.trim();
    if !value.is_empty() {
        out.push_str(&want);
        out.push_str(": ");
        out.push_str(value);
        out.push(';');
    }
    let kept = out.trim_end().len();
    out.truncate(kept);
    out
}
