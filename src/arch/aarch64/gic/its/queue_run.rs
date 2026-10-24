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
use super::queue::Queue;
use super::regs::{Regs, CREADR, CWRITER};
use super::ring_types::{Progress, Refused, BYTES, SLOT};

impl Queue {
    /// Run `cmds` and a SYNC to the collection's redistributor, and wait
    /// for the ITS to consume them all. A batch that does not finish closes
    /// the queue: every later one is refused (`ring`).
    pub(super) fn run(&mut self, regs: Regs, cmds: &[Cmd]) -> Result<(), MsiError> {
        let start = self.ring.place(cmds.len() + 1).map_err(|refused| match refused {
            Refused::TooLong => MsiError::Exhausted,
            Refused::Closed => MsiError::Hardware,
        })?;
        let Some(page) = view(&self.region) else {
            self.ring.close();
            return Err(MsiError::Hardware);
        };
        let sync = sync(self.target);
        for (n, cmd) in cmds.iter().chain(core::iter::once(&sync)).enumerate() {
            let at = (start + n as u64 * SLOT) % BYTES;
            for (w, word) in cmd.iter().enumerate() {
                let byte = at as usize + w * 8;
                page[byte..byte + 8].copy_from_slice(&word.to_le_bytes());
            }
            if !self.coherent {
                clean(&self.region, at, SLOT);
            }
        }
        // The commands are in memory before the ITS is told where they end.
        // SAFETY: a barrier; it touches no memory and changes no state.
        unsafe { core::arch::asm!("dsb ishst", options(nostack, preserves_flags)) }
        regs.write64(CWRITER, self.ring.write());
        let mut progress = Progress::Pending;
        let finished = until(|| {
            progress = self.ring.progress(regs.read64(CREADR));
            progress != Progress::Pending
        });
        if finished && progress == Progress::Done {
            return Ok(());
        }
        self.ring.close();
        Err(MsiError::Hardware)
    }
}
