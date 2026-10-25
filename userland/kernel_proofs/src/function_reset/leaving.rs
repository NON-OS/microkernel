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

//! Which claims leave, pure (K10b, K10c), kept here as the model of the
//! release K10 built: each claim taken out of the table before its device
//! retires, marked busy until the retire is done. The kernel kept the maintainers'
//! lifecycle instead (claim/stage_rule.rs): an entry stays Leaving or
//! Exiting until its last user is gone, and a claim over it, or over a wipe
//! still owed, answers Resetting, which is the property these tests hold.

extern crate alloc;

use alloc::vec::Vec;

use super::types::{Claim, ClaimError};

/// The claim on `device_id`, out of `claims` when `pid` holds it, and the
/// device marked busy until its retire is done: its epoch.
pub(crate) fn take_held(
    claims: &mut Vec<Claim>,
    busy: &mut Vec<u64>,
    pid: u32,
    device_id: u64,
) -> Result<u64, ClaimError> {
    let at = claims.iter().position(|c| c.device_id == device_id).ok_or(ClaimError::NotClaimed)?;
    if claims[at].pid != pid {
        return Err(ClaimError::NotHolder);
    }
    busy.push(device_id);
    Ok(claims.remove(at).epoch)
}

/// Every claim `pid` holds, out of `claims`, each device marked busy.
pub(crate) fn take_all(claims: &mut Vec<Claim>, busy: &mut Vec<u64>, pid: u32) -> Vec<u64> {
    let held: Vec<u64> = claims.iter().filter(|c| c.pid == pid).map(|c| c.device_id).collect();
    claims.retain(|c| c.pid != pid);
    busy.extend_from_slice(&held);
    held
}

/// Whether a new claim may take `device_id`: not while it is held, and not
/// while another claim or a retire is under way on it.
pub(crate) fn claimable(claims: &[Claim], busy: &[u64], device_id: u64) -> Result<(), ClaimError> {
    if claims.iter().any(|c| c.device_id == device_id) {
        return Err(ClaimError::AlreadyClaimed);
    }
    if busy.contains(&device_id) {
        return Err(ClaimError::Busy);
    }
    Ok(())
}

/// A claim or a retire on `device_id` is done, detach included. One mark
/// goes: a release can start on a claim just published, before the claim
/// takes its own mark off, and that release's mark stays until it is done.
pub(crate) fn done(busy: &mut Vec<u64>, device_id: u64) {
    if let Some(at) = busy.iter().position(|d| *d == device_id) {
        busy.swap_remove(at);
    }
}
