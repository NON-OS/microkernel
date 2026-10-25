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

use crate::layout::Layout;
use crate::palette::Palette;
use crate::rail::Rail;
use crate::term::prefs::Prefs;
use crate::term::state::State;

pub struct Terminal {
    pub(crate) tabs: Vec<State>,
    pub(crate) active: usize,
    pub(crate) split: Option<usize>, // the tab shown beside the active one
    pub(crate) dropped: bool, // F12: the window stands under the menubar, full width
    pub(crate) line_only: bool, // the last key changed the input line and nothing else
    pub(crate) partial: bool, // this frame repaints the input row alone (damage.rs)
    // Window width from the last paint, so the accessory clears the lights.
    pub(crate) width: u32,
    // Accessory width the frame granted, shared by the painter and hit-test.
    pub(crate) acc_w: u32,
    // Window level, so a new tab inherits the look instead of resetting it.
    pub(crate) theme: u16,
    pub(crate) font_scale: u32,
    // The record as it was last read or written, so a save carries the fields
    // this build does not surface instead of resetting them to defaults.
    pub(crate) prefs: Prefs,
    // Set only when a chrome request actually moved theme or zoom; cleared by
    // the tick that writes the record.
    pub(crate) prefs_dirty: bool,
    pub(crate) prefs_ticks: u32,
    // Live system telemetry for the rail, polled on its own tick budget
    // so the table never re-reads the kernel once per frame.
    pub(crate) rail: Rail,
    // How far the rail column has been scrolled, in pixels, clamped to the
    // content by the painter and the hit-test alike (`rail_scroll::clamp`).
    pub(crate) rail_scroll: u32,
    // The band solve from the last paint: the event path needs the rail rects
    // and cannot re-solve them, since `Metrics` reads the frame buffer.
    pub(crate) layout: Option<Layout>,
    // The command overlay. It gates the whole key path while open, so it is
    // read before the tab bindings rather than beside them.
    pub(crate) palette: Palette,
    /// Where the body's cells were drawn, and what the pointer is doing.
    pub(crate) cells: Option<super::pointer::CellGeom>,
    pub(crate) ptr: super::pointer::Pointer,
    pub(crate) key_at: i64, // when the last key came, until its frame is painted
    /// When the window started, and when it next asks the shell for a
    /// handed command (take_handed.rs, handed::cadence).
    pub(crate) started_ms: i64,
    pub(crate) handed_due_ms: i64,
}

impl Terminal {
    pub(crate) fn cur(&mut self) -> &mut State {
        &mut self.tabs[self.active]
    }
    pub(crate) fn cur_ref(&self) -> &State {
        &self.tabs[self.active]
    }
}
