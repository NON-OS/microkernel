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

use crate::browser::dom::node::Node;

use super::super::walk::Counters;
use super::counter_style::counters;
use super::function::Call;

/* Append what one content function generates on `node`: an attribute's
 * value (or the fallback), a counter in its list style, or a quote mark.
 * Counters and quotes need the walk's counter state `k`. */
pub(super) fn apply(call: Call, node: &Node, k: Option<&mut Counters>, out: &mut String) {
    match (call, k) {
        (Call::Attr(name, fallback), _) => out.push_str(node.attr(name).unwrap_or(fallback)),
        (Call::Counter(name, sep, style), Some(k)) => counters(k, name, sep, style, out),
        (Call::Quote(open, mark), Some(k)) => quote(k, open, mark, out),
        _ => {}
    }
}

/* open-quote and close-quote: curly double marks outermost, single ones
 * inside; the no- forms only move the depth. */
fn quote(k: &mut Counters, open: bool, mark: bool, out: &mut String) {
    if !open {
        k.quotes = k.quotes.saturating_sub(1);
    }
    if mark {
        let marks = if k.quotes == 0 { ['\u{201c}', '\u{201d}'] } else { ['\u{2018}', '\u{2019}'] };
        out.push(marks[usize::from(!open)]);
    }
    if open {
        k.quotes = k.quotes.saturating_add(1);
    }
}
