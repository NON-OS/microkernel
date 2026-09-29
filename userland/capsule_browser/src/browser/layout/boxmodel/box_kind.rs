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

use crate::browser::css::Computed;

use super::abs_out_of_flow::out_of_flow;
use super::tree::BoxKind;

/* The formatting context an element's box runs. An absolutely positioned
 * or fixed box is blockified, as CSS requires: it never sits in an inline
 * run, and the positioned ancestor that contains it places it. */
pub(super) fn box_kind(style: &Computed) -> BoxKind {
    if style.is_grid {
        BoxKind::Grid
    } else if style.is_flex {
        BoxKind::Flex
    } else if style.is_block || out_of_flow(style) {
        BoxKind::Block
    } else if style.is_inline_block {
        BoxKind::InlineBlock
    } else {
        BoxKind::Inline
    }
}
