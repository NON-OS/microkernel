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

use super::error::BootError;
use super::membership::membership;
use crate::record::{check, Record};
use crate::tcg::replay;

/// What the kernel admitted: the bootloader's measurement and the signed root
/// and epoch it was admitted under.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Admitted {
    pub measurement: [u8; 32],
    pub root: [u8; 32],
    pub epoch: u64,
}

fn admit(m: [u8; 32], rec: Record, trailer: &[u8]) -> Result<Admitted, BootError> {
    membership(&rec.root, &m, trailer)?;
    Ok(Admitted { measurement: m, root: rec.root, epoch: rec.epoch })
}

/*
 * With a TPM. The log must replay to the live PCR 4, so no event was dropped
 * or added; then the last application it names is what ran last before the
 * kernel, the bootloader or anything started after it. The record must be
 * signed and at or above the TPM's floor, so an old root cannot come back.
 * Then that measurement's slot under that root.
 */
pub fn measured(
    log: &[u8],
    live_pcr4: &[u8; 32],
    floor: u64,
    record: &[u8],
    trailer: &[u8],
    verify_sig: impl FnOnce(&[u8; 32], &[u8; 32], &[u8; 32]) -> bool,
) -> Result<Admitted, BootError> {
    let r = replay(log).map_err(BootError::Log)?;
    if r.pcr4 != *live_pcr4 {
        return Err(BootError::PcrMismatch);
    }
    let m = r.last_application.ok_or(BootError::NoApplication)?;
    let rec = check(record, floor, verify_sig).map_err(BootError::Record)?;
    admit(m, rec, trailer)
}

/*
 * Without a TPM: the loader file the loader itself handed over, hashed here.
 * Nothing measured it, so this says only that the bytes the loader reports are
 * enrolled; the caller must say so and never present it as a measured pass.
 * There is no floor to hold the epoch to.
 */
pub fn self_reported(
    loader: &[u8],
    record: &[u8],
    trailer: &[u8],
    verify_sig: impl FnOnce(&[u8; 32], &[u8; 32], &[u8; 32]) -> bool,
) -> Result<Admitted, BootError> {
    let m = crate::authenticode::digest(loader).map_err(BootError::Image)?;
    let rec = check(record, 0, verify_sig).map_err(BootError::Record)?;
    admit(m, rec, trailer)
}
