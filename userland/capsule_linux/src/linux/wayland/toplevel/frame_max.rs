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

//! The frame's maximise button: the window maximised, or back to its own
//! size when it is on its way to a mode already.

use crate::linux::guest::Guest;

use super::super::fit::Mode;
use super::ask::ask;

pub fn maximize(guest: &mut Guest) {
    let Some(toplevel) = guest.scene.shown.and_then(|s| s.toplevel) else { return };
    let want = match guest.scene.fit.heading() {
        Mode::Normal => Mode::Maximized,
        _ => Mode::Normal,
    };
    ask(guest, toplevel, want);
}
