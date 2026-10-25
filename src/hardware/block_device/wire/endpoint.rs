// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! One capsule serving the block wire, as its kernel mirror declares it.

use core::sync::atomic::{AtomicU32, AtomicU64};

use spin::Mutex;

use super::layout::Layout;
use crate::services::lifecycle::CapsuleState;

pub struct Endpoint {
    /// What the log calls the disk, with the service: "eMMC/SD (driver.sdhci0)".
    pub shown: &'static str,
    /// The name the kernel's requests go out under.
    pub sender: &'static str,
    /// The kernel inbox the capsule replies to.
    pub reply_inbox: &'static str,
    pub state: &'static CapsuleState,
    /// How many disks the driver may serve, units 0 up; no more than
    /// `MAX_UNITS` are asked.
    pub units: u16,
    pub slot: &'static Slot,
}

/// Units the client keeps state for, per endpoint: a UFS device's eight LUs.
pub const MAX_UNITS: usize = 8;

/// The client's own state for one endpoint, kept beside it so a mirror
/// declares everything in two statics.
pub struct Slot {
    /// One request in flight: replies are matched by id, but a second caller
    /// draining the same inbox would throw the first one's reply away.
    pub(super) call: Mutex<()>,
    /// Held across a whole write: a part-block write is read, patched and
    /// written back, and two at once would undo each other.
    pub(super) write: Mutex<()>,
    pub(super) seq: AtomicU32,
    /// Each unit's geometry, for the capsule generation it was asked of.
    pub(super) layout: Mutex<[Option<(u64, Layout)>; MAX_UNITS]>,
    /// The hash of the last line the block layer said for each unit.
    pub(super) said: [AtomicU64; MAX_UNITS],
}

impl Slot {
    pub const fn new() -> Self {
        Self {
            call: Mutex::new(()),
            write: Mutex::new(()),
            seq: AtomicU32::new(1),
            layout: Mutex::new([None; MAX_UNITS]),
            said: [const { AtomicU64::new(0) }; MAX_UNITS],
        }
    }
}

impl Default for Slot {
    fn default() -> Self {
        Self::new()
    }
}
