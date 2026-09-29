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

//! The terminal: two screens, scrollback, modes, and everything a program
//! can ask of them.

mod abs;
mod alt;
mod answer;
mod control;
mod csi;
mod csi_move;
mod csi_place;
mod cursor_style;
mod edit;
mod erase;
mod esc;
mod feed;
mod glyph;
mod handler;
mod history;
mod index;
mod join;
mod lines;
mod logical;
mod mode_state;
mod modes;
mod modes_type;
mod mouse_mode;
mod new;
mod osc;
mod pick;
mod print;
mod reflow;
mod reset;
mod resize;
mod save;
mod screen;
mod screen_modes;
mod scroll;
mod search;
mod search_match;
mod sgr;
mod sgr_attr;
mod sgr_colour;
mod soft_reset;
mod state;
mod tab_stops;
mod tabs;
mod text;
mod types;
mod view;
mod view_draw;
mod view_text;

pub use modes_type::Modes;
pub use mouse_mode::MouseMode;
pub use screen::{Cursor, Screen};
pub use state::Term;
pub use types::{ClipboardRequest, CursorShape, CursorView, Pos};
