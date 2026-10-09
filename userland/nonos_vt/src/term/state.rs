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

//! Everything the terminal holds.

use alloc::collections::VecDeque;
use alloc::string::String;
use alloc::vec::Vec;

use super::modes_type::Modes;
use super::screen::Screen;
use super::types::{ClipboardRequest, CursorShape};
use crate::charset::Charsets;
use crate::color::Palette;
use crate::line::Line;
use crate::parser::Parser;

pub struct Term {
    pub(super) cols: usize,
    pub(super) rows: usize,
    pub(super) primary: Screen,
    pub(super) alt: Screen,
    pub(super) alt_active: bool,
    pub(super) scrollback: VecDeque<Line>,
    pub(super) scrollback_limit: usize,
    /// Lines that ever left the top of the normal screen. The first line of
    /// the normal screen is absolute line `scrolled`.
    pub(super) scrolled: u64,
    /// How many lines the view is scrolled back into history.
    pub(super) view: usize,
    pub modes: Modes,
    pub(super) tabs: Vec<bool>,
    pub(super) charsets: Charsets,
    /// For REP, and for joining the character after a zero width joiner.
    pub(super) last_char: Option<char>,
    pub(super) after_zwj: bool,
    pub(super) parser: Parser,
    pub(super) replies: Vec<u8>,
    pub(super) title: String,
    pub(super) title_stack: Vec<String>,
    pub(super) title_gen: u32,
    pub(super) cwd: Option<String>,
    pub(super) clipboard: Option<ClipboardRequest>,
    /// Clipboard reads a program asked for and was refused.
    pub(super) clipboard_reads_refused: u32,
    pub(super) bell: u32,
    /// The theme the host set, and the palette programs may have changed.
    pub(super) theme: Palette,
    pub(super) palette: Palette,
    pub(super) links: Vec<String>,
    pub(super) cursor_shape: CursorShape,
    /// Absolute lines where a shell prompt started (OSC 133 A).
    pub(super) prompts: VecDeque<u64>,
    /// Rows to redraw.
    pub(super) dirty: Vec<bool>,
    /// Cell size in pixels, set by the host, for programs that ask.
    pub(super) cell_px: (u16, u16),
}
