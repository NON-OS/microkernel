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

//! Ctrl+C typed on the family's terminal: SIGINT for every process in the
//! foreground group, as Linux's tty raises it, whether or not anyone reads.

use super::family::Family;
use super::pid_space::outward;
use crate::linux::console;
use crate::linux::console::tty_rules::interrupt_line;
use crate::linux::guest::siginfo::{SigInfo, SI_KERNEL};

const SIGINT: u8 = 2;

impl Family {
    pub(super) fn settle_tty_interrupt(&mut self) {
        if !console::take_interrupt() {
            return;
        }
        // The group a shell set, else the first process's own (no job control).
        let fg = console::fg().or_else(|| self.guests.first().map(|g| outward(g.pgid)));
        let Some(fg) = fg else { return };
        let mut took = 0;
        for g in self.guests.iter_mut() {
            if g.exited.is_none() && outward(g.pgid) == fg {
                took += usize::from(g.raise_and_wake(0, SigInfo::from(SIGINT, SI_KERNEL, 0)));
            }
        }
        // Whether the Ctrl+C reached anyone, so the log can say it did.
        let line = interrupt_line(fg, took);
        let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
    }
}
