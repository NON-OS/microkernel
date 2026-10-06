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

use super::compound_part::Compound;
use super::cursor::Cur;
use super::element_args::{compound_arg, words};
use super::element_code::*;
use super::element_names::element_name;

/* A pseudo-element after '::', its name already lower-cased. */
pub(super) fn pseudo_element(
    c: &mut Cur,
    out: &mut Compound,
    name: &str,
    func: bool,
) -> Option<()> {
    out.element = if func { element_fn(c, name)? } else { element_name(name)? };
    out.spec.c += 1;
    Some(())
}

/* The four pseudo-elements that keep their CSS 2 single-colon spelling. */
pub(super) fn legacy_element(name: &str) -> Option<u8> {
    Some(match name {
        "before" => BEFORE,
        "after" => AFTER,
        "first-line" => FIRST_LINE,
        "first-letter" => FIRST_LETTER,
        _ => return None,
    })
}

fn element_fn(c: &mut Cur, name: &str) -> Option<u8> {
    Some(match name {
        "part" => words(c, true, false).map(|_| PART)?,
        "slotted" => compound_arg(c).map(|_| SLOTTED)?,
        "cue" => compound_arg(c).map(|_| CUE)?,
        "highlight" | "picker" => words(c, false, false).map(|_| OTHER)?,
        "scroll-button" => words(c, false, true).map(|_| OTHER)?,
        "view-transition-group"
        | "view-transition-image-pair"
        | "view-transition-old"
        | "view-transition-new" => words(c, false, true).map(|_| TRANSITION)?,
        _ => return None,
    })
}
