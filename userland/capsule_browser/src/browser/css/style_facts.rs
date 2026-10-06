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

//! The part of a computed style a script's getComputedStyle reads
//! (dom::style_facts), taken from the cascade's own result.

use crate::browser::dom::style_facts::{Display, Facts, Pos};

use super::computed::{Computed, Position};

/// The facts of one computed style.
pub fn facts_of(c: &Computed) -> Facts {
    let display = match c {
        c if c.display_none => Display::None,
        c if c.is_contents => Display::Contents,
        c if c.is_table_cell => Display::TableCell,
        c if c.is_table_row => Display::TableRow,
        c if c.is_table => Display::Table,
        c if c.is_grid && c.is_block => Display::Grid,
        c if c.is_grid => Display::InlineGrid,
        c if c.is_flex && c.is_block => Display::Flex,
        c if c.is_flex => Display::InlineFlex,
        c if c.is_inline_block => Display::InlineBlock,
        c if c.is_block => Display::Block,
        _ => Display::Inline,
    };
    let position = match c.position {
        _ if c.is_fixed => Pos::Fixed,
        _ if c.is_sticky => Pos::Sticky,
        Position::Static => Pos::Static,
        Position::Relative => Pos::Relative,
        Position::Absolute => Pos::Absolute,
    };
    Facts {
        display,
        hidden: c.hidden,
        position,
        color: c.color,
        bg: c.bg,
        font_px: c.font_px,
        bold: c.bold,
        italic: c.italic,
        opacity: c.opacity,
        margin: [c.margin_top, c.margin_right, c.margin_bottom, c.margin_left],
        margin_pml: c.margin_pml,
        margin_auto: [
            c.margin_left_auto || c.margin_auto_x,
            c.margin_right_auto || c.margin_auto_x,
        ],
        pad: [c.pad_top, c.pad_right, c.pad_bottom, c.pad_left],
        border: [c.border_top, c.border_right, c.border_bottom, c.border_left],
        border_box: c.border_box,
    }
}
