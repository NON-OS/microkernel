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

//! Keeping an opened volume for the boot.

use super::plan_types::Plan;
use super::say::say;
use super::state::{VolumeState, VOLUME};
use crate::fs::blockfs::BlockFsMount;
use alloc::format;

/// Keep `mount` under `key` as the open volume, and say where it lies.
pub(super) fn install(plan: &Plan, key: [u8; 32], mount: BlockFsMount) {
    let (n, at) = (plan.volume_sectors, plan.volume_base);
    say(&format!("[DATA] volume open: {n} sectors at LBA {at}"));
    *VOLUME.write() = Some(VolumeState { key, mount });
}
