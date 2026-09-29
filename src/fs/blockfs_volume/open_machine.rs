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

//! Opening the machine's data volume.
//!
//! The volume's key is derived from the TPM under a kernel label, so it is
//! never stored: this machine in this boot state gets the same key every
//! time, and any other machine or boot state gets another. The volume lies
//! where the disk plan says. It is formatted only when its header ring is
//! blank: a ring that holds sectors this key cannot open is someone's data
//! under another key or boot state, and is left alone.

use super::error::VolumeError;
use super::plan_read::read_plan;
use super::ring_blank::ring_blank;
use super::state::{VolumeState, VOLUME};
use crate::fs::blockfs::{self, BlockFsError};
use crate::security::tpm::machine_key::derive_for_kernel;

/// The kernel label the volume key is derived under.
const KEY_LABEL: &[u8] = b"blockfs.data.v1";

/// Open the data volume if it is not open yet. Every refusal is logged
/// with its reason and returned; nothing is formatted over data.
pub fn open_machine_volume() -> Result<(), VolumeError> {
    if VOLUME.read().is_some() {
        return Ok(());
    }
    let plan = read_plan()?;
    crate::fs::cryptoblock::set_window(plan.volume_base, plan.volume_sectors)
        .map_err(VolumeError::Window)?;
    let key = derive_for_kernel(KEY_LABEL).map_err(|e| {
        crate::log::warn!("[DATA] no machine key ({:?}); the data volume stays closed", e);
        VolumeError::MachineKey(e)
    })?;
    let mount = match blockfs::mount(&key) {
        Ok(m) => m,
        Err(BlockFsError::NotFormatted) if ring_blank(plan.volume_base)? => {
            let mut uuid = [0u8; 16];
            crate::crypto::rng::fill_random_bytes(&mut uuid);
            let m = blockfs::format(&key, uuid).map_err(VolumeError::BlockFs)?;
            crate::log::info!("[DATA] formatted a volume of {} sectors", plan.volume_sectors);
            m
        }
        Err(BlockFsError::NotFormatted) => {
            crate::log::warn!(
                "[DATA] the volume holds data this key cannot open; not formatting over it"
            );
            return Err(VolumeError::Unopenable);
        }
        Err(e) => return Err(VolumeError::BlockFs(e)),
    };
    crate::log::info!(
        "[DATA] volume open: {} sectors at LBA {}",
        plan.volume_sectors,
        plan.volume_base
    );
    *VOLUME.write() = Some(VolumeState { key, mount });
    Ok(())
}
