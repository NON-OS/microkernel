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

//! Ctrl+C pressed again and again. A guest may catch or ignore SIGINT, as a
//! Linux program may, so one Ctrl+C is only ever SIGINT. But three inside
//! two seconds end the foreground group with SIGKILL, which no guest can
//! catch, ignore or block: a run can always be stopped from its terminal,
//! and it still ends the ordinary way, with its box's report.

pub const PRESSES: usize = 3;
pub const WINDOW_MS: u64 = 2000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Press {
    /// SIGINT, as one Ctrl+C always is.
    Interrupt,
    /// SIGINT, and the person told that one more press ends the program.
    Warn,
    /// SIGKILL for the foreground group.
    Kill,
}

/// The presses before this one that may still count, oldest first.
#[derive(Clone, Copy, Debug, Default)]
pub struct Presses {
    at: [Option<u64>; PRESSES - 1],
}

impl Presses {
    pub const fn new() -> Self {
        Self { at: [None; PRESSES - 1] }
    }

    /// A press at `now`, in milliseconds since boot. A clock that went
    /// backwards counts as no time passing.
    pub fn press(&mut self, now: u64) -> Press {
        let inside = |t: &Option<u64>| t.is_some_and(|t| now.saturating_sub(t) <= WINDOW_MS);
        let recent = self.at.iter().filter(|t| inside(t)).count();
        if recent + 1 >= PRESSES {
            self.at = [None; PRESSES - 1];
            return Press::Kill;
        }
        self.at.rotate_left(1);
        self.at[PRESSES - 2] = Some(now);
        match recent + 1 == PRESSES - 1 {
            true => Press::Warn,
            false => Press::Interrupt,
        }
    }
}
