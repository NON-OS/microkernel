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

//! Whether a typed character still fits in a form field.
//!
//! Every field stopped taking keys at 512 bytes and said nothing, so a
//! comment written in a textarea simply stopped growing partway through a
//! sentence, while a page's own `maxlength` was not looked at at all. The
//! page's limit is now kept, quietly as every browser keeps it, since the
//! page asked for it; the browser's own bound is far larger and, when it is
//! reached, the reader is told.

use crate::browser::dom::node::Node;

/// Bytes a single-line field holds.
pub const INPUT_MAX: usize = 4 * 1024;
/// Bytes a textarea holds.
pub const TEXTAREA_MAX: usize = 64 * 1024;

/// What a typed character meets.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Room {
    /// It goes in.
    Fits,
    /// The page's `maxlength` is reached: refused without a word, as the
    /// page asked.
    PageLimit,
    /// The browser's own bound, in bytes, is reached: refused and said.
    BrowserLimit(usize),
}

/// Whether `c` fits after `value` in the field `node`.
pub fn room(node: &Node, value: &str, c: char) -> Room {
    /* maxlength counts characters; a value that is not a whole number
     * sets no limit, as in HTML. */
    let page = node.attr("maxlength").and_then(|m| m.trim().parse::<usize>().ok());
    if page.is_some_and(|max| value.chars().count() >= max) {
        return Room::PageLimit;
    }
    let max = if node.tag == "textarea" { TEXTAREA_MAX } else { INPUT_MAX };
    if value.len() + c.len_utf8() > max {
        return Room::BrowserLimit(max);
    }
    Room::Fits
}
