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

use alloc::vec::Vec;

use crate::browser::css::{Computed, WhiteSpace};

use super::box_kind::box_kind;
use super::contexts::field_size::size_field;
use super::field_label::{field_label, widest_option};
use super::leaf::leaf;
use super::tree::{BoxKind, BoxNode};
use super::walk::{ElementIn, Walk};

/* Box for an <input> or <select>: an atomic inline, like an inline-block,
 * unless its display makes it a block, sized as the control is (see
 * field_size.rs) where CSS leaves a side auto, and carrying its visible
 * label (value, placeholder or chosen option) as one unwrapped line of
 * text. Hidden inputs render nothing. */
#[inline(never)]
pub(super) fn element_field(w: &Walk, item: &ElementIn, mut style: Computed) -> Option<BoxNode> {
    if item.c.attr("type").is_some_and(|t| t.eq_ignore_ascii_case("hidden")) {
        return None;
    }
    let widest = (item.c.tag == "select").then(|| widest_option(w.dom, item.ch));
    size_field(item.c, &mut style, widest.as_deref());
    let label = field_label(w.dom, item.ch);
    let mut kids: Vec<BoxNode> = Vec::new();
    if !label.is_empty() {
        let mut text = leaf(BoxKind::Text(label), &style, &None, item.ch);
        text.style.white_space = WhiteSpace::Pre;
        kids.push(text);
    }
    let kind = match box_kind(&style) {
        BoxKind::Inline => BoxKind::InlineBlock,
        k => k,
    };
    Some(BoxNode {
        kind,
        style,
        href: None,
        dom_id: item.ch,
        bg_image: None,
        grid_place: None,
        children: kids,
        aux: Default::default(),
    })
}
