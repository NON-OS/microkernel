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


//! A live boot's volume. The disk this boot came from carries the NONOS
//! store but no data plan, so no volume of this machine's is on it: the
//! volume is made in RAM instead, under a key drawn for this boot and kept
//! nowhere else. Models and anything else written to it last until power
//! off, and none of it reaches a disk.

use alloc::format;

use super::error::VolumeError;
use super::say::say;
use super::state::{VolumeState, VOLUME};
use crate::fs::blockfs;
use crate::fs::cryptoblock::{ram, set_window};

/// The least a session volume is made with: below it, nothing worth
/// fetching would fit.
const LEAST: u64 = 256 << 20;

pub(super) fn open_session_volume() -> Result<(), VolumeError> {
    let mut volume = VOLUME.write();
    if volume.is_some() {
        return Ok(());
    }
    let room = crate::memory::phys::free_memory().saturating_sub(ram::reserve());
    if room < LEAST {
        say(&format!(
            "[DATA] live boot: {} MiB free beyond the {} MiB kept for the system; no volume in memory",
            room >> 20,
            ram::reserve() >> 20
        ));
        return Err(VolumeError::NoMemory);
    }
    let sectors = room / crate::fs::cryptoblock::SECTOR_BYTES as u64;
    ram::start(sectors).map_err(VolumeError::Device)?;
    set_window(0, sectors).map_err(VolumeError::Window)?;
    let mut key = [0u8; 32];
    crate::crypto::rng::fill_random_bytes(&mut key);
    let mut uuid = [0u8; 16];
    crate::crypto::rng::fill_random_bytes(&mut uuid);
    let mount = blockfs::format(&key, uuid).map_err(VolumeError::BlockFs)?;
    say(&format!(
        "[DATA] live boot: this session's volume is in memory, up to {} MiB, gone at power off",
        room >> 20
    ));
    *volume = Some(VolumeState { key, mount });
    Ok(())
}
