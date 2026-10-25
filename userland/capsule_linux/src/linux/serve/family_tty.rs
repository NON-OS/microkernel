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
//! foreground group, as Linux's tty raises it, whether or not anyone reads;
//! SIGKILL once it is pressed three times inside two seconds
//! (console/intr_press.rs), so a guest that ignores SIGINT is still ended.

use super::family::Family;
use super::pid_space::outward;
use crate::linux::console;
use crate::linux::console::intr_press::Press;
use crate::linux::console::tty_rules::interrupt_line;
use crate::linux::guest::siginfo::{SigInfo, SI_KERNEL};

const SIGINT: u8 = 2;
const SIGKILL: u8 = 9;

const WARN: &[u8] = b"linux: press Ctrl+C once more within 2 seconds to end the program\n";

impl Family {
    pub(super) fn settle_tty_interrupt(&mut self) {
        if !console::take_interrupt() {
            return;
        }
        // The group a shell set, else the first process's own (no job control).
        let fg = console::fg().or_else(|| self.guests.first().map(|g| outward(g.pgid)));
        let Some(fg) = fg else { return };
        let press = console::press();
        let (signo, name) = match press {
            Press::Kill => (SIGKILL, "SIGKILL"),
            _ => (SIGINT, "SIGINT"),
        };
        let mut took = 0;
        for g in self.guests.iter_mut() {
            if g.exited.is_none() && outward(g.pgid) == fg {
                took += usize::from(g.raise_and_wake(0, SigInfo::from(signo, SI_KERNEL, 0)));
            }
        }
        if press == Press::Warn {
            crate::linux::say::say(WARN);
        }
        // Whether the Ctrl+C reached anyone, so the log can say it did.
        let line = interrupt_line(name, fg, took);
        let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
    }
}
