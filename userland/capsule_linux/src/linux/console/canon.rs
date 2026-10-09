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

//! New terminal settings, and telling the NONOS terminal when a guest takes
//! its tty out of canonical mode or back (tty_rules), so a shell's own line
//! editor gets keys raw and the terminal stops echoing them too.

use super::say::say;
use super::state::with;
use super::termios::Termios;
use super::tty_rules::mode_switch;

/// New settings, kept as given; a change of canonical mode is told.
pub fn set_termios(raw: &[u8]) {
    let Ok(t) = <Termios>::try_from(raw) else {
        return;
    };
    let was = with(|c| core::mem::replace(&mut c.termios, t));
    if let Some(seq) = mode_switch(&was, &t) {
        say(seq);
    }
}
