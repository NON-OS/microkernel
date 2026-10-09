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

//! Making a terminal, and the full reset a program can ask for.

use alloc::collections::VecDeque;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use super::modes_type::Modes;
use super::resize::clamp_size;
use super::screen::Screen;
use super::state::Term;
use super::tabs::default_tabs;
use super::types::CursorShape;
use crate::charset::Charsets;
use crate::color::Palette;
use crate::limits::MAX_SCROLLBACK;
use crate::parser::Parser;

impl Term {
    pub fn new(cols: usize, rows: usize, scrollback: usize) -> Term {
        let (cols, rows) = clamp_size(cols, rows);
        Term {
            cols,
            rows,
            primary: Screen::new(cols, rows),
            alt: Screen::new(cols, rows),
            alt_active: false,
            scrollback: VecDeque::new(),
            scrollback_limit: scrollback.min(MAX_SCROLLBACK),
            scrolled: 0,
            view: 0,
            modes: Modes::default(),
            tabs: default_tabs(cols),
            charsets: Charsets::default(),
            last_char: None,
            after_zwj: false,
            parser: Parser::new(),
            replies: Vec::new(),
            title: String::new(),
            title_stack: Vec::new(),
            title_gen: 0,
            cwd: None,
            clipboard: None,
            clipboard_reads_refused: 0,
            bell: 0,
            theme: Palette::xterm(),
            palette: Palette::xterm(),
            links: Vec::new(),
            cursor_shape: CursorShape::Block,
            prompts: VecDeque::new(),
            dirty: vec![true; rows],
            cell_px: (0, 0),
        }
    }
}
