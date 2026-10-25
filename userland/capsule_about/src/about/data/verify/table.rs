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

use alloc::vec::Vec;

/// Every capsule's granted mask, kept so a caller can ask several questions of
/// one instant instead of re-reading the table per question.
pub struct Tally {
    pub(super) masks: Vec<u64>,
    pub(super) names: Vec<Vec<u8>>,
    pub total: u32,
    pub unmasked: u32,
    pub own_mask: u64,
}

impl Tally {
    pub fn holders(&self, mask: u64) -> u32 {
        self.masks.iter().filter(|m| *m & mask != 0).count() as u32
    }

    /// Holders of `mask` whose name `named` does not list.
    pub fn beyond(&self, mask: u64, named: &[&[u8]]) -> u32 {
        let held = self.masks.iter().copied().zip(self.names.iter().map(|n| &n[..]));
        super::held_by::beyond(held, mask, named)
    }

    /// Capsules that can log and also see keys, input or a secret.
    pub fn secret_loggers(&self) -> u32 {
        super::held_by::secret_loggers(self.masks.iter().copied())
    }
}
