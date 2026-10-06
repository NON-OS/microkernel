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

use crate::browser::css::{Computed, Float, WhiteSpace};

use super::super::abs_out_of_flow::out_of_flow;
use super::super::leaf::leaf;
use super::super::tree::{BoxKind, BoxNode};
use super::super::walk::Walk;

/* A text child of the element styled `parent`. Text with anything but
 * white space is kept whole; inline layout collapses its spaces. A
 * white-space-only node is kept where it means something: always under
 * pre, pre-wrap and pre-line (it holds preserved spaces and newlines),
 * and otherwise, as the single space it collapses to, only after inline
 * content or at the start of an inline box, where it separates words
 * ("<b>a</b> <i>b</i>"). Between blocks it makes no box at all. */
#[inline(never)]
pub(in super::super) fn text_child(
    w: &mut Walk,
    t: &str,
    ch: usize,
    parent: &Computed,
    link: &Option<String>,
    out: &mut Vec<BoxNode>,
) {
    let blank = t.bytes().all(|b| matches!(b, b' ' | b'\t' | b'\n' | b'\r' | b'\x0C'));
    let collapses = matches!(parent.white_space, WhiteSpace::Normal | WhiteSpace::Nowrap);
    let text = match (blank, collapses) {
        (false, _) | (true, false) => t.to_string(),
        (true, true) => {
            let in_flow = |b: &&BoxNode| !out_of_flow(&b.style) && b.style.float == Float::None;
            let after_inline = match out.iter().rev().find(in_flow) {
                Some(prev) => !prev.kind.block_level(),
                None => {
                    !(parent.is_block || parent.is_inline_block || parent.is_flex || parent.is_grid)
                }
            };
            if !after_inline {
                return;
            }
            String::from(" ")
        }
    };
    *w.count += 1;
    out.push(leaf(BoxKind::Text(text), parent, link, ch));
}
