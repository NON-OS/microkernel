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

use super::held::Held;
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
    /// The serial `sent` was first told under; told again (again), it keeps
    /// it, and an ack of any serial from it to the latest is of the same
    /// configure.
    pub(super) sent_from: u32,
    /// The edges the resize `sent` asks for holds (held.rs); None for any
    /// other configure.
    pub(super) sent_held: Option<Held>,
    /// The same for `acked`.
    pub(super) acked_held: Option<Held>,
    /// Acked, and waiting for the buffer drawn for it.
    pub(super) acked: Option<Configure>,
    /// The last configure asked, kept once it is shown, which a configure
    /// that changes only the states says again.
    pub(super) last: Option<Configure>,
}

impl Fit {
    /// The mode the window is in now.
    pub fn now(&self) -> Mode {
        self.mode
    }

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
        self.sent_from = serial;
        self.sent_held = None;
        self.last = Some(told);
        Some(told)
    }

    /// The window's mode and size told again under `serial`, as a change of
    /// its other states is: a configure not yet acked takes the new serial,
    /// and an ack of this one or of any it was told under before is taken
    /// for it (steps.rs); a window in its own size is told 0 by 0, which
    /// keeps it.
    pub fn again(&mut self, serial: u32) -> Configure {
        if let Some(c) = self.sent.as_mut() {
            c.serial = serial;
            self.last = Some(*c);
            return *c;
        }
        let mode = self.acked.map_or(self.mode, |c| c.mode);
        match self.last.filter(|c| c.mode == mode && mode.fills()) {
            Some(c) => Configure { serial, ..c },
            None => Configure { serial, mode, rect: None },
        }
    }
}
