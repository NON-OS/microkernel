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

use crate::fs::blockfs::{BlockFsMount, ReadCache};
use crate::services::lifecycle::TripLock;

pub(super) struct VolumeState {
    pub(super) key: [u8; 32],
    pub(super) mount: BlockFsMount,
}

/*
 * The open volume. A trip lock: its holder keeps it across the disk's round
 * trips and may sleep or be killed there; a holder that never runs again is
 * swept by the next caller. Set or cleared only under the write hold, and
 * nothing keeps a reference into it across a give.
 */
pub(super) static VOLUME: TripLock<Option<VolumeState>> = TripLock::new(None, let_go);

/// A writer leaving, given or swept: what it held for the device is dropped
/// unsent. Runs under the rows or the write hold, so it must never wait.
fn let_go(_: &mut Option<VolumeState>, swept: bool) {
    crate::fs::cryptoblock::drop_held(swept);
}

/// A pid handed to a new process: every row the old one left in the
/// volume's lock goes, a writer's held sectors with it. Runs at purge.
pub(crate) fn forget_holder(pid: u32) {
    VOLUME.forget(pid);
}

/// The volume's read cache. Its entries name the volume and the count of
/// writes made to it, so another volume, a moved window or any write leaves
/// nothing to reuse.
pub(super) static READS: ReadCache = ReadCache::new();
