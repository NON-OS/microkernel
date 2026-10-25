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
//! The one word an AP and the boot CPU decide its start on: what it holds,
//! and the reads anyone may make. Who may move it, and when, is in
//! `boot_race` (the AP's claim against the boot CPU giving up) and
//! `boot_park` (parking an AP that claimed and then missed its deadline).
//!
//! Host-checked by path from `kernel_proofs`; it uses nothing but core.

use core::sync::atomic::{AtomicU32, Ordering};

pub(super) const WAITING: u32 = 0;
pub(super) const ENTERED: u32 = 1;
pub(super) const ABANDONED: u32 = 2;
pub(super) const PARK_ASKED: u32 = 3;
pub(super) const PARKED: u32 = 4;

pub struct BootClaim(pub(super) AtomicU32);

impl BootClaim {
    pub const fn new() -> Self {
        Self(AtomicU32::new(WAITING))
    }

    /// Reset before the AP is sent its STARTUP. Only the boot CPU calls this,
    /// and only for a slot it has never handed out.
    pub fn arm(&self) {
        self.0.store(WAITING, Ordering::Release);
    }

    /// The AP claimed its start: whatever has been asked of it since.
    pub fn entered(&self) -> bool {
        matches!(self.0.load(Ordering::Acquire), ENTERED | PARK_ASKED | PARKED)
    }
}

impl Default for BootClaim {
    fn default() -> Self {
        Self::new()
    }
}
