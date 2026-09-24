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

//! Starting the write: entropy for the identifiers, the plan against the
//! chosen disk, and the first job. Nothing touches the disk here; a plan
//! that fails leaves the disk as it was and says why.

use alloc::string::String;

use nonos_blk_client::DeviceSink;
use nonos_disk::{Plan, Session};
use nonos_libc::{crypto_random, mk_time_millis};

use super::work::Job;
use crate::install::state::State;

pub fn start(state: &mut State) -> Result<(), String> {
    let image = state.image.as_ref().ok_or_else(|| String::from("no image to write"))?;
    let device = state
        .selected_disk()
        .and_then(|d| d.device)
        .ok_or_else(|| String::from("that disk has no working driver"))?;

    let mut entropy = [0u8; 36];
    let rc = crypto_random(entropy.as_mut_ptr(), entropy.len());
    if rc < 0 {
        return Err(alloc::format!(
            "the kernel gave no entropy for the disk identifiers (errno {rc})"
        ));
    }

    let plan = Plan::new(device.sectors, &image.as_disk_image(), entropy).map_err(describe)?;
    let now = mk_time_millis().max(0) as u64;
    state.job = Some(Job::writing(DeviceSink { device }, Session::new(plan), now));
    Ok(())
}

pub fn describe(e: nonos_disk::WriteError) -> String {
    use nonos_disk::WriteError::*;
    match e {
        DiskTooSmall { total_sectors, needed_sectors } => alloc::format!(
            "the disk holds {} and the image needs {}",
            crate::install::format::bytes(total_sectors * 512),
            crate::install::format::bytes(needed_sectors * 512)
        ),
        Volume(v) => alloc::format!("the volume could not be laid out ({v:?})"),
        Sink(s) => alloc::format!("the disk refused a transfer (status {})", s.0),
        Mismatch { lba } => {
            alloc::format!("sector {lba} read back different from what was written")
        }
    }
}
