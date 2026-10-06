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

//! What Linux does with a signal no handler takes, when its disposition is
//! SIG_DFL. The table is Linux's, transcribed.

/// What an uncaught signal does when its disposition is SIG_DFL.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Default {
    Ignore,
    Terminate,
    Stop,
    Continue,
}

/// Linux's table: child status, urgent data and window size are ignored,
/// SIGCONT continues, the four stop signals stop, everything else ends the
/// process. The core-dumping ones end it too: no core is ever written, so the
/// status carries no core flag, as on Linux with a zero core limit.
pub fn default_of(signum: u8) -> Default {
    match signum {
        17 | 23 | 28 => Default::Ignore,
        18 => Default::Continue,
        19..=22 => Default::Stop,
        _ => Default::Terminate,
    }
}
