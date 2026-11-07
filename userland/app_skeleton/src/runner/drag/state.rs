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
//! What a press on the frame holds while the pointer moves.

use nonos_toolkit::decorations::{DecorationHit, Edges, Placed};

use super::super::press_part::PressGrab;

/// The shape last told before any was: the first hover tells the arrow, so a
/// window states its own shape and never shows one of a window under it.
const NOT_TOLD: u8 = u8::MAX;

pub(in crate::runner) struct DragState {
    pub active: bool,
    pub hover: DecorationHit,
    pub press: PressGrab,
    pub(in crate::runner) press_x: i32,
    pub(in crate::runner) press_y: i32,
    pub(in crate::runner) base_x: u32,
    pub(in crate::runner) base_y: u32,
    /// The edges a resize holds, the rect it started from, and where it is.
    pub(in crate::runner) resizing: Option<(Edges, Placed, Placed)>,
    /// The pointer's shape the frame under it wants, and the one last asked
    /// of the compositor (window_ops/tell_cursor.rs).
    pub(in crate::runner) cursor: (u8, u8),
    /// The display's size as the compositor gave it at this drag's first
    /// step: every later step of the same drag is clamped to it, not asked
    /// again (window_ops/move_window.rs).
    pub(in crate::runner) display: Option<(u32, u32)>,
    /// The last press on the title bar, the first of a double click
    /// (double.rs), and the runner's clock as the event at hand came; None
    /// when the runner did not say, and then no press is the second.
    pub(in crate::runner) title_press: Option<super::TitlePress>,
    pub(in crate::runner) clock_ms: Option<i64>,
}

impl DragState {
    pub(in crate::runner) const fn new() -> Self {
        Self {
            active: false,
            hover: DecorationHit::None,
            press: PressGrab::new(),
            press_x: 0,
            press_y: 0,
            base_x: 0,
            base_y: 0,
            resizing: None,
            cursor: (0, NOT_TOLD),
            display: None,
            title_press: None,
            clock_ms: None,
        }
    }

    /// The display's size for a step of this drag: the one read at its first
    /// step, or `read`'s answer when none was. A failed read is not kept, so
    /// the next step asks again.
    pub(in crate::runner) fn display_or(
        &mut self,
        read: impl FnOnce() -> Option<(u32, u32)>,
    ) -> Option<(u32, u32)> {
        if self.display.is_none() {
            self.display = read();
        }
        self.display
    }

    /// The window was put at `at` under a drag whose pointer is at `local`
    /// in the press's frame: later steps move it from there by what the
    /// pointer moves (unmaximize.rs).
    pub(in crate::runner) fn rebase(&mut self, local: (i32, i32), at: (u32, u32)) {
        (self.press_x, self.press_y) = local;
        (self.base_x, self.base_y) = at;
    }
}

pub(in crate::runner) enum PointerAction {
    /// Not the frame's: the event goes on to the app.
    None,
    /// The frame's, with nothing to apply: a press on an edge, the motion of
    /// a resize, its release where it began. The app never sees it, so the
    /// row under an edge is not clicked by the press that resizes.
    Held,
    MoveTo(u32, u32),
    /// A drag of the right or bottom edge: the origin stays.
    ResizeTo(u32, u32),
    /// A drag that moved the left or top edge: origin and size together.
    Place(Placed),
    HoverChanged,
    /// A double click on the title bar: maximized, or restored.
    ToggleMaximize,
    /// A drag of a maximized window's title bar went far enough to take it
    /// out of full screen, held there (unmaximize.rs).
    Unmaximize(super::Grip),
}
