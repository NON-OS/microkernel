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

//! One pseudo-terminal: two byte queues joined by one set of settings. What
//! the master writes goes through the line discipline to the slave; what
//! the slave writes goes through output processing to the master. Pure.

use alloc::collections::VecDeque;

use super::flags::{lflag, Termios, COOKED, ICANON};
use super::input::Input;

pub struct Pair {
    /// The slave's settings, which a request on either side reads and sets,
    /// as Linux's tty_mode_ioctl has the master's act on its slave.
    pub termios: Termios,
    pub input: Input,
    /// What the master may read: the slave's output and the echo.
    pub output: VecDeque<u8>,
    /// Set until unlockpt (TIOCSPTLCK 0); a locked slave cannot be opened.
    pub locked: bool,
    /// `struct winsize`, as TIOCSWINSZ last set it.
    pub winsize: [u8; 8],
    /// The foreground group, in the guests' numbering; 0 for none.
    pub fg: u32,
    /// The session it is the controlling terminal of; 0 for none.
    pub session: u32,
    /// Whether a master, and a slave, descriptor is open anywhere in the
    /// family; whether a slave ever was; whether the master has gone.
    pub master: bool,
    pub slave: bool,
    pub slave_seen: bool,
    pub hung: bool,
}

impl Pair {
    /// Locked, with Linux's cooked settings, its master just opened.
    pub fn opened() -> Pair {
        let (output, input) = (VecDeque::new(), Input::default());
        let (locked, master, slave, slave_seen, hung) = (true, true, false, false, false);
        let (termios, winsize, fg, session) = (COOKED, [0; 8], 0, 0);
        Pair {
            termios,
            input,
            output,
            locked,
            winsize,
            fg,
            session,
            master,
            slave,
            slave_seen,
            hung,
        }
    }

    pub fn canon(&self) -> bool {
        lflag(&self.termios) & ICANON != 0
    }
}
