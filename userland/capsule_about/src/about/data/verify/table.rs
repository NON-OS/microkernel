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

//! One read of the kernel process table, tallied.

// The same ceiling `runtime::sample` reads with, so the two screens describe the
// same table rather than two different truncations of it.
pub(super) const MAX_PROCS: usize = 64;

pub(super) const INIT_NAME: &[u8] = b"init";

/// Every capsule's granted mask, kept so a caller can ask several questions of
/// one instant instead of re-reading the table per question.
pub struct Tally {
    pub(super) masks: [u64; MAX_PROCS],
    pub(super) is_init: [bool; MAX_PROCS],
    pub total: u32,
    pub unmasked: u32,
    pub own_mask: u64,
}

impl Tally {
    pub fn holders(&self, mask: u64) -> u32 {
        self.masks[..self.total as usize].iter().filter(|m| *m & mask != 0).count() as u32
    }

    /// Holders other than init.
    pub fn stray(&self, mask: u64) -> u32 {
        let n = self.total as usize;
        (0..n).filter(|i| self.masks[*i] & mask != 0 && !self.is_init[*i]).count() as u32
    }
}
