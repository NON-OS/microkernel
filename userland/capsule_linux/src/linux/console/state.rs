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

//! The family's one terminal: whether it is on one, and what that one has
//! been sent and set to.

use core::cell::RefCell;

use nonos_libc::mk_tty_query;

use super::queue::Queue;
use super::termios::{Termios, COOKED};
use crate::linux::run_mode::Mode;

pub(super) struct Console {
    /// Set once seen, never cleared: a family on a terminal stays private.
    pub(super) attached: bool,
    pub(super) queue: Queue,
    pub(super) termios: Termios,
    /// What a guest last set with TIOCSWINSZ (`winsize`).
    pub(super) winsize: super::winsize::Set,
}

pub(super) struct Cell(RefCell<Console>);

/*
 * SAFETY: one personality process serves one family from one serve loop,
 * answering one call at a time, so no two borrows can overlap.
 */
unsafe impl Sync for Cell {}

static CONSOLE: Cell = Cell(RefCell::new(Console {
    attached: false,
    queue: Queue::new(),
    termios: COOKED,
    winsize: None,
}));

pub(super) fn with<T>(f: impl FnOnce(&mut Console) -> T) -> T {
    f(&mut CONSOLE.0.borrow_mut())
}

/// Called before the guest's first byte: a `cli` run is on the terminal
/// that started it from the start.
pub fn enter(mode: Mode) {
    if mode == Mode::Cli {
        with(|c| c.attached = true);
    }
}

/// True when the family is on a terminal: a `cli` run, or one any of whose
/// streams the kernel says reaches a terminal. A terminal that sends both
/// input and output to files (`< f > g`) still shows stderr.
pub fn attached() -> bool {
    if with(|c| c.attached) {
        return true;
    }
    let on = (0..3).any(|n| mk_tty_query(n).is_some());
    if on {
        with(|c| c.attached = true);
    }
    on
}

/// What was typed and not read, dropped: TCSETSF, and TCFLSH of input.
pub fn flush() {
    with(|c| c.queue.clear());
}

/// Whether standard stream `n` (0, 1 or 2) is the terminal. A `cli` run's
/// three are. On a terminal's `linux` command each is what the kernel says,
/// asked each time: the terminal leaves a stream redirected to or from a
/// file out, so a program asking isatty writes plain bytes there.
pub fn on_tty(n: u32) -> bool {
    if !attached() {
        return false;
    }
    if !crate::linux::terminal::started() {
        return true;
    }
    mk_tty_query(n).is_some()
}

/// stdout goes to a file and stderr to the screen, over the one channel:
/// each message then starts with the stream it is from (`TAG_STDOUT`).
pub fn split() -> bool {
    crate::linux::terminal::started() && mk_tty_query(1).is_none() && mk_tty_query(2).is_some()
}
