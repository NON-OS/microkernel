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
use alloc::vec::Vec;

use crate::browser::css::Computed;

use super::super::element_field::element_field;
use super::super::element_img::element_img;
use super::super::element_svg::element_svg;
use super::super::leaf::leaf;
use super::super::tree::{BoxKind, BoxNode};
use super::super::walk::{ElementIn, Walk};

/* The elements laid out as one leaf box: <br>, <img>, <svg> and form
 * fields. Out of line, so their boxes take no room in each frame of the
 * recursive tree build; false for every other element. */
#[inline(never)]
pub(super) fn replaced(
    w: &mut Walk,
    item: &ElementIn,
    parent: &Computed,
    link: &Option<String>,
    style: &Computed,
    out: &mut Vec<BoxNode>,
) -> bool {
    let style = *style;
    match item.c.tag.as_str() {
        /* Hard line break: a newline the inline layout honors. */
        "br" => out.push(leaf(BoxKind::Text(String::from("\n")), parent, link, item.ch)),
        "img" => out.push(element_img(w, item, parent, link, style)),
        "svg" => out.push(element_svg(w, item, parent, link, style)),
        "input" | "select" => out.extend(element_field(w, item, style)),
        _ => return false,
    }
    true
}
