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
//! where the disk plan says. A volume whose key header says a passphrase
//! keys it is left for `passphrase_volume`; the TPM is not asked.

use super::error::VolumeError;
use super::key_header::Keyed;
use super::key_header_io::read_key_header;
use super::mount_or_format::mount_or_format;
use super::opened::install;
use super::plan_read::read_plan;
use super::plan_types::PlanError;
use super::session::open_session_volume;
use super::state::VOLUME;
use crate::hardware::block_device::BlockDeviceError;
use crate::security::tpm::machine_key::derive_for_kernel;

/// The kernel label the volume key is derived under.
const KEY_LABEL: &[u8] = b"blockfs.data.v1";

/// Open the data volume if it is not open yet. Every refusal is logged
/// with its reason and returned; nothing is formatted over data.
pub fn open_machine_volume() -> Result<(), VolumeError> {
    if VOLUME.read().is_some() {
        return Ok(());
    }
    /*
     * A disk the block layer chose carries the store or a plan. With no
     * plan it is a live stick, never an installed disk, so nothing of this
     * machine's is on it and the volume is held in RAM. Any other refusal,
     * a disk not ready among them, is said as it is.
     */
    let plan = match read_plan() {
        Err(VolumeError::Plan(PlanError::NoPlan)) => return open_session_volume(),
        /*
         * No disk the kernel drives, on a boot the loader read the store for:
         * a live stick no driver of the kernel's brought up. Its volume is
         * held in RAM as any live stick's is.
         */
        Err(VolumeError::Device(BlockDeviceError::Dead))
            if crate::hardware::block_device::store_copy::present() =>
        {
            return open_session_volume();
        }
        other => other?,
    };
    if plan.is_live() {
        return open_session_volume();
    }
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
    let mount = mount_or_format(&key, &plan, &Keyed::Tpm)?;
    install(&plan, key, mount);
    Ok(())
}
