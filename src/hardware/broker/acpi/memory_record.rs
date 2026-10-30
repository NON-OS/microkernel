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

//! An ACPI SystemMemory window as an entry in the broker's MMIO grant table,
//! where MkMmioUnmap and the capsule's exit find it like any other.

use crate::hardware::broker::grant::{self, MmioGrant};
use crate::hardware::broker::quota_log;

/// The device id ACPI grants carry in the MMIO table: no device has it, so
/// releasing a device never takes an ACPI window with it.
pub const ACPI_DEVICE: u64 = u64::MAX;

pub struct MemoryGrant {
    /// Where the requested first byte is mapped in the caller.
    pub user_va: u64,
    pub grant_id: u64,
}

/// Record the whole pages mapped at `user_va`, which is what unmapping
/// gives back, and return the grant's id; None, logged, when the caller's
/// grant quota is spent and nothing was recorded.
pub(super) fn record(pid: u32, start: u64, end: u64, user_va: u64) -> Option<u64> {
    let grant_id = grant::allocate_id();
    // No claim stands behind it, but it counts against the quota.
    let entered = grant::insert_unclaimed(MmioGrant {
        grant_id,
        pid,
        device_id: ACPI_DEVICE,
        claim_epoch: 0,
        bar_index: 0,
        physical_start: start,
        user_va,
        length: end - start,
        flags: 0,
    });
    if let Err(spent) = entered {
        quota_log::refused(pid, "ACPI", spent);
        return None;
    }
    Some(grant_id)
}
