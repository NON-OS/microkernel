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

use alloc::string::{String, ToString};
use alloc::vec::Vec;

mod join;

use super::scan::split_top;
use join::join;

/* Complex selectors one nested prelude may expand to. */
const MAX_EXPANDED: usize = 256;
/* Bytes one expanded prelude may take in all. Each '&' copies the whole
 * parent, so the text can grow geometrically with depth; a nested rule
 * that would pass this bound, or join::MAX_JOINED for one selector, is
 * dropped, as an invalid rule is. */
const MAX_PRELUDE: usize = 64 << 10;

/* The complex selectors a rule's prelude stands for, as text (CSS Nesting
 * 1). At top level they are the list itself. Inside a parent rule every
 * parent selector combines with every nested one: each & (or :scope in an
 * @scope block) becomes the parent, and a nested selector with no & is a
 * descendant of the parent, or relative to it when it starts with a
 * combinator. A parent that is a compound or leads the nested selector
 * reads exactly as :is(parent) would. Empty when the expansion is too
 * large to keep, which drops the rule. */
pub(super) fn resolve(prelude: &str, parents: Option<&[String]>) -> Vec<String> {
    let children = split_top(prelude, b',');
    let Some(parents) = parents else {
        return children.map(|c| c.to_string()).collect();
    };
    let children: Vec<&str> = children.collect();
    let (mut out, mut bytes) = (Vec::new(), 0usize);
    for p in parents {
        for c in &children {
            if out.len() >= MAX_EXPANDED {
                return out;
            }
            let Some(s) = join(p, c) else { return Vec::new() };
            bytes += s.len();
            if bytes > MAX_PRELUDE {
                return Vec::new();
            }
            out.push(s);
        }
    }
    out
}
