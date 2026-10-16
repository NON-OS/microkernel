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

//! Which claims leave, pure (K10b): taken out of the table before their
//! devices retire, so a teardown of the same holder that runs meanwhile,
//! from a thread that faulted on a revoked BAR, finds none to retire again.
//! kernel_proofs::function_reset.

extern crate alloc;

use alloc::vec::Vec;

use super::types::{Claim, ClaimError};

/// The claim on `device_id`, out of `claims` when `pid` holds it: its epoch.
pub(crate) fn take_held(
    claims: &mut Vec<Claim>,
    pid: u32,
    device_id: u64,
) -> Result<u64, ClaimError> {
    let at = claims.iter().position(|c| c.device_id == device_id).ok_or(ClaimError::NotClaimed)?;
    if claims[at].pid != pid {
        return Err(ClaimError::NotHolder);
    }
    Ok(claims.remove(at).epoch)
}

/// Every claim `pid` holds, out of `claims`: their devices.
pub(crate) fn take_all(claims: &mut Vec<Claim>, pid: u32) -> Vec<u64> {
    let held = claims.iter().filter(|c| c.pid == pid).map(|c| c.device_id).collect();
    claims.retain(|c| c.pid != pid);
    held
}
