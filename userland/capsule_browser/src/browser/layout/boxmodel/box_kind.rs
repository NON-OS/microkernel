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

use crate::browser::css::{Computed, Float};

use super::abs_out_of_flow::out_of_flow;
use super::tree::{BoxKind, BoxNode};

/* The formatting context an element's box runs. inline-flex and
 * inline-grid are inline-level: an InlineBlock atom on the line whose
 * inside is a flex or grid context (layout_box routes it by is_flex and
 * is_grid). An absolutely positioned, fixed or floated box is blockified,
 * as CSS requires: it never sits in an inline run. */
pub(super) fn box_kind(style: &Computed) -> BoxKind {
    let blockified = style.is_block || out_of_flow(style) || style.float != Float::None;
    if style.is_grid && blockified {
        BoxKind::Grid
    } else if style.is_flex && blockified {
        BoxKind::Flex
    } else if style.is_grid || style.is_flex || style.is_inline_block {
        if blockified {
            BoxKind::Block
        } else {
            BoxKind::InlineBlock
        }
    } else if blockified {
        BoxKind::Block
    } else {
        BoxKind::Inline
    }
}

/* A box whose children are flex or grid items: a flex or grid container,
 * or an inline-flex or inline-grid atom. */
pub(super) fn makes_items(kind: &BoxKind, style: &Computed) -> bool {
    match kind {
        BoxKind::Flex | BoxKind::Grid => true,
        BoxKind::InlineBlock => style.is_flex || style.is_grid,
        _ => false,
    }
}

/* Whether a container's child takes part in its flex or grid layout: an
 * in-flow block-level box or an image (text runs were wrapped as blocks
 * when the tree was built). */
pub(super) fn is_item(c: &BoxNode) -> bool {
    !out_of_flow(&c.style) && (c.kind.block_level() || matches!(c.kind, BoxKind::Image { .. }))
}

/* The formatting context a box runs inside: an inline-flex or inline-grid
 * atom runs flex or grid as a block-level container does. */
pub(super) fn inner(n: &BoxNode) -> &BoxKind {
    match n.kind {
        BoxKind::InlineBlock if n.style.is_flex => &BoxKind::Flex,
        BoxKind::InlineBlock if n.style.is_grid => &BoxKind::Grid,
        _ => &n.kind,
    }
}
