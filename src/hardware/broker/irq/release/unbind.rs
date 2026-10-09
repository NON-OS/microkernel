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

//! Revocation paths: explicit `MkIrqUnbind`, exit
//! teardown, and `MkDeviceRelease`. The unwind shape depends on
//! the grant kind:
//!
//!   * INTx — mask the IO-APIC line, deactivate the broker slot,
//!     free the slot back into the bitmap. The IO-APIC redirection
//!     entry is left programmed (idempotent and the line is
//!     masked); the next bind for the same GSI overwrites it.
//!   * MSI-X — mask the per-vector entry, zero the table entry so
//!     a stale message cannot be re-armed, deactivate and free the
//!     broker slot. When the unwind drops the last MSI-X grant
//!     for the device, the kernel issues a full disable so the
//!     device-side enable bit returns to its post-reset state.

use super::super::records::{self, drain_for_device, drain_for_pid};
use super::super::types::IrqError;
use super::unwind::teardown;

pub fn unmap_grant(pid: u32, grant_id: u64) -> Result<(), IrqError> {
    let g = records::remove(pid, grant_id)?;
    teardown(&g);
    Ok(())
}

pub fn release_for_device(pid: u32, device_id: u64) -> usize {
    let drained = drain_for_device(pid, device_id);
    let count = drained.len();
    for g in &drained {
        teardown(g);
    }
    count
}

pub fn release_all_for_pid(pid: u32) -> usize {
    let drained = drain_for_pid(pid);
    let count = drained.len();
    for g in &drained {
        teardown(g);
    }
    count
}
