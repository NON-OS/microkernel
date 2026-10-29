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

//! When a process's end is followed by the calls before it (recent_say.rs):
//! a nonzero exit, and an end by SIGKILL, which is how a run the person
//! stopped from its terminal ends (console/intr_press.rs). A program stuck
//! where no signal it may catch reaches it then shows what it was stuck in,
//! where before it showed only that it ended. A clean exit says no more,
//! and a death on another signal is said where the thread died. Pure.

const SIGKILL: i32 = 9;

/// `code` the exit's status byte, `signo` the signal that ended it, or 0.
pub fn worth(code: i32, signo: i32) -> bool {
    (signo == 0 && code != 0) || signo == SIGKILL
}
