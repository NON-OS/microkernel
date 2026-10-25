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

//! Saying a blank (G-89): a driver that cleared its screen for a silent
//! owner says so once per silence, so a hung compositor is a line on the
//! log and not only a black screen. The line is the driver's to say; this
//! keeps when.

use super::display::Display;
use super::platform::Platform;
use super::server::Server;

/// The line a driver says, through its own debug channel.
pub const BLANK_LINE: &[u8] = b"[DISPLAY] compositor silent 3 s: screen cleared\n";

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Tell {
    /// The owner is heard, or no blank has run since.
    #[default]
    Quiet,
    /// A blank ran in this silence and is not yet said.
    Due,
    /// Said for this silence.
    Told,
}

impl Tell {
    pub(super) fn ran(&mut self) {
        if *self == Tell::Quiet {
            *self = Tell::Due;
        }
    }
}

impl<D: Display, P: Platform> Server<D, P> {
    /// True once per silence, after its first blank: say BLANK_LINE now.
    pub fn take_tell(&mut self) -> bool {
        let due = self.tell == Tell::Due;
        if due {
            self.tell = Tell::Told;
        }
        due
    }

    /// `watch`, then BLANK_LINE handed to `say` once per silence.
    pub fn watch_said(&mut self, now_ms: u64, say: impl FnOnce(&'static [u8])) {
        self.watch(now_ms);
        if self.take_tell() {
            say(BLANK_LINE);
        }
    }
}
