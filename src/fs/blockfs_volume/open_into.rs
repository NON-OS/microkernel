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

//! The machine volume's open, under the volume's write hold: the window,
//! the key header, the key derived from the TPM, and the mount, installed
//! into the held volume. Nothing here takes the volume again.

use super::super::error::VolumeError;
use super::super::key_header::Keyed;
use super::super::key_header_io::read_key_header;
use super::super::mount_or_format::mount_or_format;
use super::super::opened::install;
use super::super::plan_types::Plan;
use super::super::state::VolumeState;
use crate::security::tpm::machine_key::derive_for_kernel;

/// The kernel label the volume key is derived under.
const KEY_LABEL: &[u8] = b"blockfs.data.v1";

pub(in super::super) fn open_into(
    v: &mut Option<VolumeState>,
    plan: &Plan,
) -> Result<(), VolumeError> {
    crate::fs::cryptoblock::set_window(plan.volume_base, plan.volume_sectors)
        .map_err(VolumeError::Window)?;
    if let Some(Keyed::Passphrase(_)) = read_key_header()? {
        crate::log::warn!("[DATA] the volume is keyed by a passphrase; it waits for one");
        return Err(VolumeError::NeedsPassphrase);
    }
    let key = derive_for_kernel(KEY_LABEL).map_err(|e| {
        crate::log::warn!("[DATA] no machine key ({:?}); the data volume stays closed", e);
        VolumeError::MachineKey(e)
    })?;
    let mount = mount_or_format(&key, plan, &Keyed::Tpm)?;
    install(v, plan, key, mount);
    Ok(())
}
