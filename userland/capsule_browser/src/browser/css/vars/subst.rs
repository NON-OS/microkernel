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

use alloc::borrow::Cow;
use alloc::rc::Rc;
use alloc::string::String;

use super::light_dark::strip_light_dark;
use super::{find_close, split_top_comma};

/* Fallbacks nested in fallbacks followed before a value is invalid. */
const MAX_DEPTH: u32 = 32;
/* A value longer than this after substitution is invalid: it stops a few
 * properties referencing each other twice over from growing without end. */
pub(super) const MAX_LEN: usize = 64 * 1024;

/* Where var() finds a custom property's value. The value comes back
 * already substituted; None is no value (undefined or invalid). */
pub(crate) trait Lookup {
    fn var(&mut self, name: &str) -> Option<Rc<str>>;
}

/* `value` with every var() replaced and every light-dark(a, b) collapsed
 * to its light side (this browser renders the light scheme). Borrowed
 * when there is nothing to replace. None when a var() has no value and no
 * fallback: the declaration is invalid. */
pub(crate) fn substitute<'v>(v: &'v str, l: &mut dyn Lookup) -> Option<Cow<'v, str>> {
    let v = match v.contains("var(") {
        true => Cow::Owned(expand(v, l, 0)?),
        false => Cow::Borrowed(v),
    };
    match v.contains("light-dark(") {
        true => Some(Cow::Owned(strip_light_dark(&v))),
        false => Some(v),
    }
}

fn expand(value: &str, vars: &mut dyn Lookup, depth: u32) -> Option<String> {
    if depth >= MAX_DEPTH {
        return None;
    }
    let (mut out, mut cursor) = (String::new(), 0);
    while let Some(rel) = value[cursor..].find("var(") {
        let p = cursor + rel;
        out.push_str(&value[cursor..p]);
        let Some(close) = find_close(value, p + 4) else {
            out.push_str(&value[p..]);
            return Some(out);
        };
        let (name, fallback) = split_top_comma(&value[p + 4..close]);
        match vars.var(name.trim()) {
            Some(v) => out.push_str(&v),
            None => out.push_str(&expand(fallback?.trim(), vars, depth + 1)?),
        }
        cursor = close + 1;
        if out.len() > MAX_LEN {
            return None;
        }
    }
    out.push_str(&value[cursor..]);
    Some(out)
}
