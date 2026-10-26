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
#[path = "open_into.rs"]
mod open_into;
use super::plan_read::read_plan;
use super::plan_types::PlanError;
use super::session::open_session_volume;
use super::state::VOLUME;
use crate::hardware::block_device::BlockDeviceError;

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
        Err(VolumeError::Plan(PlanError::NoPlan)) => return open_session_volume(false),
        /*
         * No disk the kernel drives, on a boot the loader read the store for:
         * a live stick no driver of the kernel's brought up. Its volume is
         * held in RAM as any live stick's is.
         */
        Err(VolumeError::Device(BlockDeviceError::Dead))
            if crate::hardware::block_device::store_copy::present() =>
        {
            return open_session_volume(false);
        }
        other => other?,
    };
    if let Some(opened) = super::session::keep_switch::in_memory(plan.is_live()) {
        return opened;
    }
    /*
     * One write hold from the check to the install: two callers cannot both
     * open, and a holder killed part-way is swept with nothing installed.
     * Taken only here, after the in-memory decision, which takes the stream
     * lock and the volume itself (keep_switch).
     */
    let mut v = VOLUME.write();
    if v.is_some() {
        return Ok(());
    }
    open_into::open_into(&mut v, &plan)
}
