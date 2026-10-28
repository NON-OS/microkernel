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

//! What the family holds open, noted before each answer as the pipes' ends
//! are: a pair no descriptor names is let go, and one whose master has gone
//! is hung up, as Linux's pty_close hangs up the slave. Pure.

use super::table::{Table, To};

pub const SIGHUP: u8 = 1;
pub const SIGCONT: u8 = 18;

impl Table {
    /// `open` gives each pseudo-terminal descriptor in the family as its
    /// number and whether it is a master. A hang-up is SIGHUP and SIGCONT
    /// for the session's leader, and the pair controls no session after.
    pub fn note(&mut self, open: impl Iterator<Item = (u32, bool)>) {
        let mut seen = alloc::vec![(false, false); self.pairs.len()];
        for (n, master) in open {
            if let Some(s) = seen.get_mut(n as usize) {
                match master {
                    true => s.0 = true,
                    false => s.1 = true,
                }
            }
        }
        let Table { pairs, signals } = self;
        for (slot, (master, slave)) in pairs.iter_mut().zip(seen) {
            let Some(p) = slot else { continue };
            if p.master && !master && !p.hung {
                p.hung = true;
                if p.session != 0 {
                    signals.extend([
                        (To::Leader(p.session), SIGHUP),
                        (To::Leader(p.session), SIGCONT),
                    ]);
                }
                p.session = 0;
            }
            (p.master, p.slave) = (master, slave);
            if !master && !slave {
                *slot = None;
            }
        }
    }
}
