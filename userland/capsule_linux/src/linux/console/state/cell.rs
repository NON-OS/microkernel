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

//! The family's one console, kept where every call on it finds it.

use core::cell::RefCell;

use super::super::queue::Queue;
use super::super::termios::{Termios, COOKED};

pub(in super::super) struct Console {
    /// Set once seen, never cleared: a family on a terminal stays private.
    pub(in super::super) attached: bool,
    pub(in super::super) queue: Queue,
    pub(in super::super) termios: Termios,
    /// What a guest last set with TIOCSWINSZ (`winsize`).
    pub(in super::super) winsize: super::super::winsize::Set,
}

struct Cell(RefCell<Console>);

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

pub(in super::super) fn with<T>(f: impl FnOnce(&mut Console) -> T) -> T {
    f(&mut CONSOLE.0.borrow_mut())
}
