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

mod at_rule;
mod block;
mod container;
mod cost;
mod cx;
mod element;
mod layer;
mod nesting;
mod ops;
mod property;
pub(super) mod scan;
mod scope;
mod select;
mod supports;
mod verdict;

use alloc::vec::Vec;

use super::decls::parse_decl;
use super::matching_brace::matching_brace;
use block::{block, flush, Parent};
pub(super) use cx::Cx;
use scan::item_end;

/* Nesting of style rules and conditional blocks followed; deeper blocks
 * are skipped whole, which bounds the parser's recursion. */
const MAX_DEPTH: u32 = 16;

/* Append the rules of `src`: a whole sheet at top level, or the block of a
 * style rule (`parent`) holding declarations, nested rules and nested
 * conditional at-rules. Each item ends at a top-level ';' or block. */
pub(super) fn parse_into(src: &str, cx: &mut Cx, depth: u32, parent: Option<&Parent>) {
    let (mut rest, mut own) = (src, Vec::new());
    while !cx.full() {
        rest = rest.trim_start();
        if rest.is_empty() {
            break;
        }
        let stop = item_end(rest, parent.is_some());
        let (head, tail) = (rest[..stop].trim(), &rest[stop..]);
        if let Some(open) = tail.strip_prefix('{') {
            let end = matching_brace(open);
            rest = open.get(end + 1..).unwrap_or("");
            flush(&mut own, parent, cx);
            block(head, &open[..end], cx, depth, parent);
            continue;
        }
        rest = tail.get(1..).unwrap_or("");
        if head.starts_with('@') {
            at_rule::statement(head, cx);
        } else if let Some(d) = parent.and(parse_decl(head)) {
            own.push(d);
        }
    }
    flush(&mut own, parent, cx);
}
