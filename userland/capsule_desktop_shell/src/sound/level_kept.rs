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

//! The volume in force as the service last said it, asked once per shell.

use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use nonos_audio_proto::{volume_query, MasterVolume, HDR_LEN};

use super::volume::{REQUEST_ID, SET_TIMEOUT_MS};
use super::volume_call::call;
use crate::state::volume::Heard;

// The volume in force as the service last said it, and whether it has said it
// since the shell started.
static LEVEL: AtomicU32 = AtomicU32::new(MasterVolume::FULL.level as u32);
static MUTED: AtomicBool = AtomicBool::new(false);
static LEARNT: AtomicBool = AtomicBool::new(false);

/// The volume to step from: the service's, asked once per shell, so a shell
/// started after the level was turned down does not step from full.
pub(super) fn in_force(port: u32) -> Result<MasterVolume, Heard> {
    if LEARNT.load(Ordering::Relaxed) {
        let level = LEVEL.load(Ordering::Relaxed) as u8;
        return Ok(MasterVolume { level, muted: MUTED.load(Ordering::Relaxed) });
    }
    let mut req = [0u8; HDR_LEN];
    let n = volume_query(&mut req, REQUEST_ID);
    let now = call(port, &req[..n], SET_TIMEOUT_MS)?;
    keep(now);
    Ok(now)
}

pub(super) fn keep(volume: MasterVolume) {
    LEVEL.store(u32::from(volume.level), Ordering::Relaxed);
    MUTED.store(volume.muted, Ordering::Relaxed);
    LEARNT.store(true, Ordering::Relaxed);
}
