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

/*
 * What the Qwen step holds: this machine's memory, which tiers fit it for
 * an installed NONOS and for a session in memory (default.rs says which,
 * by the fetcher's own rule, and which row starts), whether a NONOS disk
 * loaded and whether this is QEMU's software CPU, and the row the person
 * chose, if they chose one. Only the tiers that fit are rows.
 *
 * Setup runs only where no answers were kept, and an installed disk is
 * written with them, so a NONOS disk loaded here is a stick: amnesic on it
 * is a session whose volume the kernel holds in memory.
 */

use alloc::vec::Vec;

use super::default::{default_row, offered, For};
use super::memory::memory;
use super::pins::PINNED;
use super::tcg::under_tcg;

pub struct QwenState {
    /* Bytes of memory; None when the kernel would not say, and then no tier fits. */
    pub memory: Option<u64>,
    /* Indices of PINNED that fit to run on an installed NONOS, and in a session's memory. */
    pub fit: Vec<u8>,
    pub session_fit: Vec<u8>,
    /* A NONOS disk loaded this boot: the store settled without error. */
    pub disk: bool,
    /* The store has not settled yet, so whether there is a disk is not known. */
    pub pending: bool,
    /* QEMU's software CPU (TCG). */
    pub tcg: bool,
    /* The row the person moved to: 0 none, i + 1 the i-th tier offered; None, not yet. */
    pub picked: Option<u8>,
}

impl QwenState {
    pub fn read() -> Self {
        let memory = memory();
        let fit = offered(PINNED, memory, For::Installed);
        let session_fit = offered(PINNED, memory, For::Session);
        let (disk, tcg) = (crate::keep::store_ready(), under_tcg());
        let pending = !disk && crate::keep::store_pending();
        Self { memory, fit, session_fit, disk, pending, tcg, picked: None }
    }

    /*
     * Ask again while the store has not settled. Setup reads this once as it
     * starts, and under QEMU's software CPU it starts before vfs has loaded
     * the disk, so the step said there was none for the whole of setup.
     * Whether the answer changed, so the step is drawn again.
     */
    pub fn refresh(&mut self) -> bool {
        if !self.pending {
            return false;
        }
        self.disk = crate::keep::store_ready();
        self.pending = !self.disk && crate::keep::store_pending();
        !self.pending
    }

    /* Who the tier is for, with the Mode step installing or not. */
    pub fn who(&self, install: bool) -> For {
        match (install, self.disk) {
            (true, _) => For::Installed,
            (false, true) => For::Session,
            (false, false) => For::NoDisk,
        }
    }

    /* The tiers offered, as indices of PINNED, smallest first. */
    pub fn offered(&self, install: bool) -> &[u8] {
        match self.who(install) {
            For::Installed => &self.fit,
            For::Session => &self.session_fit,
            For::NoDisk => &[],
        }
    }

    /* How many tier rows may be chosen. */
    pub fn open(&self, install: bool) -> u8 {
        self.offered(install).len() as u8
    }

    /* How many tiers are not listed because they need more memory than there is. */
    pub fn hidden(&self, install: bool) -> usize {
        match self.who(install) {
            For::NoDisk => 0,
            _ => PINNED.len() - self.offered(install).len(),
        }
    }

    /* The row shown chosen: the person's while it is open, else the default. */
    pub fn row(&self, install: bool) -> u8 {
        let who = self.who(install);
        let start = || default_row(PINNED, self.offered(install), self.memory, who);
        let row = self.picked.unwrap_or_else(start);
        if row > self.open(install) {
            0
        } else {
            row
        }
    }

    /* The tier at row `row` (1 and on), with its files' summed length. */
    pub fn tier_at(&self, install: bool, row: u8) -> Option<(&'static [u8], u64)> {
        let i = (row as usize).checked_sub(1)?;
        let at = *self.offered(install).get(i)?;
        PINNED.get(at as usize).copied()
    }

    /* The chosen tier's name, as the Terminal's qwen takes it. */
    pub fn chosen(&self, install: bool) -> Option<&'static [u8]> {
        self.tier_at(install, self.row(install)).map(|(tier, _)| tier)
    }
}
