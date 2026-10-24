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

//! Whether a write may change the disk's store, as a pure rule.
//!
//! Only the installer writes a disk plan that keeps a volume, and it does so
//! only after the person chose to install. So a disk carrying such a plan is
//! one the person asked to keep things on, and every other disk is a boot
//! that keeps nothing. On those the store may only be cleared: a sector of
//! zeros erases, it cannot record. A disk whose plan could not be read is
//! treated as keeping nothing, so a fault never opens the store.
//!
//! No crate paths here, so the host proofs mount this file as it is.

/// What the disk the store lives on says about this boot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Disk {
    /// A plan with a data volume: an installed disk.
    Installed,
    /// No plan, or a live stick's plan with no volume.
    Live,
    /// The plan could not be read or was refused.
    Unread,
}

/// True when `bytes` may be written to the store on a boot whose disk is
/// `disk`.
pub(crate) fn may_write(disk: Disk, bytes: &[u8]) -> bool {
    match disk {
        Disk::Installed => true,
        Disk::Live | Disk::Unread => bytes.iter().all(|b| *b == 0),
    }
}

/// How a plan read maps to a `Disk`: `Some(true)` a plan with a volume,
/// `Some(false)` none or a live one, `None` a read or check that failed.
pub(crate) fn disk_of(plan_keeps_volume: Option<bool>) -> Disk {
    match plan_keeps_volume {
        Some(true) => Disk::Installed,
        Some(false) => Disk::Live,
        None => Disk::Unread,
    }
}
