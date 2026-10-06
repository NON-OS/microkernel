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

//! Bringing a model, or any large file, onto the data volume, verified.
//!
//! The digest it must have is the caller's: a signed capsule pins it in its
//! own code, so the capsule's measurement covers it. The disk only says
//! where the bytes wait. The file is linked under its name only after its
//! digest matched, so a bad import never becomes a file; its sealed blocks
//! stay allocated, since the volume only allocates forward.

use super::error::VolumeError;
use super::hex::{as_str, hex32};
use super::import_one::import_from;
use super::import_record::recorded;
use super::imported::Imported;
use super::open_machine::open_machine_volume;
use super::plan_read::read_plan;
use super::plan_types::PlanError;
use super::remove_import::forget_record;
use crate::fs::blockfs::BlockFsError;
use crate::hardware::block_device::BlockDeviceError;

/// Import `name` from whichever of the plan's files is `want_bytes` long and
/// hashes to `want`. The length picks the candidates, so a wrong file is
/// told apart before it costs a hash.
pub fn import(name: &[u8], want: &[u8; 32], want_bytes: u64) -> Result<Imported, VolumeError> {
    open_machine_volume()?;
    let recorded = match recorded(name, want) {
        /* A record with no file under its name: a removal cut short. */
        Err(VolumeError::BlockFs(BlockFsError::NotFound)) => {
            forget_record(name)?;
            None
        }
        other => other?,
    };
    if let Some(bytes) = recorded {
        /* The digest went to the console once, when the model was sealed.
         * Saying it at every open would log which model is in use each time. */
        super::say::say("[DATA] a pinned model on the volume, its record verified");
        return Ok(Imported { bytes, sha256: *want, fresh: false });
    }
    /* A file under the name with no record: taken, and nothing vouches for it. */
    if super::stat::stat(name).is_ok() {
        crate::log::warn!("[DATA] import refused: the name holds a file no record vouches for");
        return Err(VolumeError::NameTaken);
    }
    /*
     * A volume in RAM is a live boot's. A live stick may still carry files to
     * import (the release stick's model tier), named by a live plan; without
     * a plan there is nothing laid on the disk to bring in.
     */
    let plan = match read_plan() {
        Err(VolumeError::Plan(PlanError::NoPlan)) if crate::fs::cryptoblock::ram::on() => {
            return Err(VolumeError::NoImport);
        }
        /* A volume in RAM over no disk the kernel drives: nothing to import from. */
        Err(VolumeError::Device(BlockDeviceError::Dead)) if crate::fs::cryptoblock::ram::on() => {
            return Err(VolumeError::NoImport);
        }
        other => other?,
    };
    let mut refused = None;
    for &(at, bytes) in plan.imports().iter().filter(|&&(_, b)| b == want_bytes) {
        match import_from(name, want, at, bytes) {
            Err(VolumeError::DigestMismatch) => refused = Some(VolumeError::DigestMismatch),
            done => return done,
        }
    }
    if refused.is_none() {
        crate::log::warn!(
            "[DATA] import refused: none of the plan's {} files is {} bytes, sha256 {}",
            plan.count,
            want_bytes,
            as_str(&hex32(want))
        );
    }
    Err(refused.unwrap_or(VolumeError::NoImport))
}
