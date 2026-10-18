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

//! A window's way into and out of full screen, a configure at a time. Pure,
//! so the host proofs hold it.

use super::mode::{Configure, Mode, Rect};

/// Where a guest window is on its way between sizes.
#[derive(Default)]
pub struct Fit {
    /// The mode the shown window is in.
    pub(super) mode: Mode,
    /// The rect the window had before it filled the display.
    pub(super) saved: Option<Rect>,
    /// Told to the guest and not yet acked.
    pub(super) sent: Option<Configure>,
    /// Acked, and waiting for the buffer drawn for it.
    pub(super) acked: Option<Configure>,
}

impl Fit {
    /// The mode the window is on its way to: the last one told, else its own.
    pub fn heading(&self) -> Mode {
        self.sent.or(self.acked).map_or(self.mode, |c| c.mode)
    }

    /// Ask for `want`, with the window at `now` and the display's full-screen
    /// rect `fill`. The configure to send, under `serial`, or None when the
    /// window is already on its way there.
    pub fn ask(
        &mut self,
        want: Mode,
        now: Option<Rect>,
        fill: Rect,
        serial: u32,
    ) -> Option<Configure> {
        let heading = self.heading();
        if want == heading {
            return None;
        }
        // Kept once, as it leaves its own size, and not again on the way back.
        if self.mode == Mode::Normal && heading == Mode::Normal {
            self.saved = now;
        }
        let rect = if want.fills() { Some(fill) } else { self.saved };
        let told = Configure { serial, mode: want, rect };
        self.sent = Some(told);
        Some(told)
    }
}
