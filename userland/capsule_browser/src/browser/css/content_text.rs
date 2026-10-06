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

mod args;
mod call;
mod counter_style;
mod function;
mod numerals;
mod string;

use alloc::string::String;

use crate::browser::dom::node::Node;

use super::walk::Counters;
use function::function;
use string::string_token;

/* Generated text kept from one content value. */
const MAX_TEXT: usize = 4096;

/* The text a content value generates on `node`'s pseudo-element: its
 * strings (CSS escapes decoded, the hex form icon fonts use included),
 * attr() values, counter() and counters() in their list styles and the
 * quote marks, in order. The alternative text after '/' is for speech
 * and is dropped. None for none and normal, which generate no box; an
 * empty string still makes one. url() and gradient images draw nothing
 * here and are skipped. */
pub(super) fn content_text(
    value: &str,
    node: &Node,
    mut counters: Option<&mut Counters>,
) -> Option<String> {
    let v = value.trim();
    if v.eq_ignore_ascii_case("none") || v.eq_ignore_ascii_case("normal") {
        return None;
    }
    let mut out = String::new();
    let mut rest = v;
    while let Some(&b) = rest.as_bytes().first() {
        if out.len() > MAX_TEXT || b == b'/' {
            break;
        }
        let used = if b == b'"' || b == b'\'' {
            let (text, used) = string_token(rest);
            out.push_str(&text);
            used
        } else {
            let (call, used) = function(rest);
            call::apply(call, node, counters.as_deref_mut(), &mut out);
            used
        };
        rest = rest.get(used..).unwrap_or("").trim_start();
    }
    Some(out)
}
