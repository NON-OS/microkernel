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

//! Where the data volume's sectors may go, as pure rules.
//!
//! A live stick keeps no volume on its disk, so its volume is in RAM for the
//! boot. An installed disk's volume is opened only on a boot that keeps;
//! until then the volume is in RAM as a live stick's is, and when keeping
//! turns on that RAM volume is dropped, not copied: what it holds was
//! written while the person had not chosen to keep anything.
//!
//! No crate paths here, so the host proofs mount this file as it is.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Place {
    Ram,
    Disk,
}

/// Where the volume opens: `installed` when the disk plan names a volume
/// on this disk.
pub(crate) const fn place(installed: bool, keeping: bool) -> Place {
    if installed && keeping {
        Place::Disk
    } else {
        Place::Ram
    }
}

/// When keeping turns on: drop the open volume, only when it is a RAM
/// volume held in place of an installed disk's. A live stick's stays.
pub(crate) const fn drop_at_arm(held_for_disk: bool) -> bool {
    held_for_disk
}

/// Whether a sealed sector may be written: always to RAM, to a disk only
/// on a boot that keeps.
pub(crate) const fn may_write(ram: bool, keeping: bool) -> bool {
    ram || keeping
}
