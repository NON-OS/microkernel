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

//! How often a pad that may have reset is set up again (reset.rs): at most
//! MAX_TRIES set-ups that took with no touch report between, and HOLDOFF_MS
//! from the end of one set-up to the start of the next, so a pad that will
//! not keep the mode is read as the mouse it stays, a pad not yet ready for
//! writes is asked once a second until it is, and a pointing stick on its
//! mouse collection costs one read-back a second however slow the bus.

use super::mouse_run::MouseRun;

const MAX_TRIES: u8 = 3;
const HOLDOFF_MS: u64 = 1_000;

#[derive(Clone, Copy, Default)]
pub struct Recover {
    /// Touchpad mode is to hold on the pad: it has an Input Mode to write.
    pub touch_mode: bool,
    /// The pad's Input Mode did not read back at its last set-up.
    pub mode_unread: bool,
    /// A reset was rung for and no set-up of the pad has taken since.
    pub owed: bool,
    /// The mouse reports since the last touch report.
    pub run: MouseRun,
    tries: u8,
    due_ms: u64,
}

impl Recover {
    /// A touch report was decoded: touchpad mode holds, nothing is owed.
    pub fn held(&mut self) {
        (self.tries, self.owed) = (0, false);
        self.run.touched();
    }

    /// Whether the pad may be set up now: tries left and the holdoff past.
    pub(super) fn may(&self) -> bool {
        self.tries < MAX_TRIES && now_ms() >= self.due_ms
    }

    /// A set-up ended, and the holdoff starts. One whose writes took pays
    /// what is owed and, if it was a try at a pad that reset (`a_try`),
    /// spends one. One the pad or the bus refused spends nothing: only the
    /// holdoff spaces it from the next.
    pub(super) fn ran(&mut self, took: bool, a_try: bool) {
        if took {
            self.owed = false;
            self.tries = self.tries.saturating_add(u8::from(a_try));
        }
        self.due_ms = now_ms().saturating_add(HOLDOFF_MS);
    }
}

pub(super) fn now_ms() -> u64 {
    nonos_libc::mk_uptime_ms().max(0) as u64
}
