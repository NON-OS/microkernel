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

//! The lid, as the shell says it moved: closed darkens every output and
//! then presents nothing, open draws the whole screen again. Pure, so the
//! frames it allows are proven on the host (compositor_proofs lid_tests).
//! Darkening is a frame of its own, composed black through the same path as
//! any frame, so a monitor still on shows nothing of the desktop; only once
//! all of it went out is the lid dark and the tick idle. Waking damages the
//! whole canvas, so the privacy band and the trusted mark come back drawn
//! whole, never from what the screen held before.

use super::cursor::CursorState;

/// What a closed lid shows on every output.
pub const LID_BLACK: u32 = 0xFF00_0000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lid {
    Open,
    /// Closed, the black frame still going out.
    Darkening,
    /// Closed, the black frame out: nothing is presented.
    Dark,
}

impl Lid {
    /// The lid after the shell says `closed`, and whether the whole canvas
    /// is damaged, to be drawn black or drawn again.
    pub fn moved(self, closed: bool) -> (Lid, bool) {
        match (self, closed) {
            (Lid::Open, true) => (Lid::Darkening, true),
            (Lid::Darkening | Lid::Dark, false) => (Lid::Open, true),
            (same, _) => (same, false),
        }
    }

    /// Whether a frame composes black instead of the scene.
    pub fn black(self) -> bool {
        self != Lid::Open
    }

    /// The pointer as the cursor plane shows it: hidden while closed.
    pub fn pointer(self, mut now: CursorState) -> CursorState {
        now.visible &= !self.black();
        now
    }

    /// Whether a frame may present at all.
    pub fn presents(self) -> bool {
        self != Lid::Dark
    }

    /// The lid after a frame, `refused` when the display turned some of it
    /// away (kept damaged): the black frame is out once all of it went.
    pub fn after_frame(self, refused: bool) -> Lid {
        match self {
            Lid::Darkening if !refused => Lid::Dark,
            same => same,
        }
    }
}
