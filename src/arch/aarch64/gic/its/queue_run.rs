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

use super::super::msi::MsiError;
use super::super::wait::until;
use super::encode::cmd::Cmd;
use super::encode::route::sync;
use super::memory::{clean, view};
use super::queue::{Queue, BYTES, MAX_BATCH, SLOT};
use super::regs::{Regs, CREADR, CREADR_STALLED, CWRITER, QUEUE_OFFSET};

impl Queue {
    /// Run `cmds` and a SYNC to the collection's redistributor, and wait
    /// for the ITS to consume them all.
    pub(super) fn run(&mut self, regs: Regs, cmds: &[Cmd]) -> Result<(), MsiError> {
        if cmds.len() >= MAX_BATCH {
            return Err(MsiError::Exhausted);
        }
        let page = view(&self.region).ok_or(MsiError::Hardware)?;
        for cmd in cmds.iter().chain(core::iter::once(&sync(self.target))) {
            let at = self.write as usize;
            for (n, word) in cmd.iter().enumerate() {
                page[at + n * 8..at + n * 8 + 8].copy_from_slice(&word.to_le_bytes());
            }
            if !self.coherent {
                clean(&self.region, self.write, SLOT);
            }
            self.write = (self.write + SLOT) % BYTES;
        }
        // The commands are in memory before the ITS is told where they end.
        // SAFETY: a barrier; it touches no memory and changes no state.
        unsafe { core::arch::asm!("dsb ishst", options(nostack, preserves_flags)) }
        regs.write64(CWRITER, self.write);
        let mut stalled = false;
        let done = until(|| {
            let read = regs.read64(CREADR);
            stalled = read & CREADR_STALLED != 0;
            stalled || read & QUEUE_OFFSET == self.write
        });
        if done && !stalled {
            Ok(())
        } else {
            Err(MsiError::Hardware)
        }
    }
}
