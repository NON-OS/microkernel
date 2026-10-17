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

//! The compare-and-swap loops on the FACS lock word.

use crate::exec::{Fault, FaultKind, Host, Machine};
use crate::region::{acquire_step, release_step};

/// How long to wait between looks at a lock firmware holds. Firmware tells
/// of its release through an SCI (GBL_RLS); until the SCI is wired, looking
/// again every millisecond, under the evaluation's time budget, is the wait.
const POLL_US: u64 = 1000;
/// Looks at the word when giving the lock back; our choice, from no source.
const GIVE_LOOKS: u32 = 8;

impl<H: Host> Machine<'_, H> {
    /// A platform without a FACS has no firmware to share the lock with, so
    /// the lock is the capsule's at once.
    pub(crate) fn lock_take(&mut self, at: u32) -> Result<(), Fault> {
        let mut old = 0u32;
        loop {
            self.tick(at)?;
            let (new, takes) = acquire_step(old);
            match self.host.lock_word(old, new) {
                None => return Ok(()),
                Some(found) if found != old => old = found,
                Some(_) if takes => return Ok(()),
                // Marked pending; firmware holds it. Wait, then look again.
                Some(_) => {
                    self.pause(POLL_US, at)?;
                    old = new;
                }
            }
        }
    }

    /// Never refused by the budget: an evaluation stopped for running out
    /// of steps or time must still hand the lock back, or firmware waits on
    /// it forever. Firmware changes a word the capsule owns only by marking
    /// it pending, so a few looks always do.
    pub(crate) fn lock_give(&mut self, at: u32) -> Result<(), Fault> {
        let mut old = 2u32;
        for _ in 0..GIVE_LOOKS {
            let (new, pending) = release_step(old);
            match self.host.lock_word(old, new) {
                None => return Ok(()),
                Some(found) if found != old => old = found,
                Some(_) => {
                    if pending {
                        self.host.lock_released();
                    }
                    return Ok(());
                }
            }
        }
        Err(self.fault(FaultKind::Mutex, at))
    }
}
