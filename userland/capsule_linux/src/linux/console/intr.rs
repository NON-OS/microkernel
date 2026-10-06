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

//! Ctrl+C on the terminal: with ISIG set, the VINTR byte is not input but a
//! SIGINT for the foreground group, and what was typed before it is dropped,
//! as Linux's n_tty does without NOFLSH. Watched by the serve loop, so a
//! guest that never reads still hears it.

use super::input::pull;
use super::state::{attached, on_tty, with};
use super::tty_rules::{holds, interrupt_byte};

/// True when Ctrl+C was typed for a terminal that raises signals; the
/// queue is then emptied.
pub fn take_interrupt() -> bool {
    if !attached() {
        return false;
    }
    /* Asked before the console is borrowed, which on_tty borrows too. */
    let tty = on_tty(0);
    let Some(intr) = with(|c| interrupt_byte(&c.termios, tty)) else {
        return false;
    };
    pull();
    let hit = with(|c| holds(&c.queue, intr));
    if hit {
        with(|c| c.queue.clear());
    }
    hit
}
