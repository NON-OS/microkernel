// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::consts::{RC_NO_TPM, RC_NV_UNINITIALIZED};
use super::define::nv_define_counter;
use super::increment::nv_increment;
use super::read::nv_read_counter;
use crate::security::tpm_extend::tpm_present;
use uefi::table::boot::BootServices;

/*
 * Whether the TPM's NV counter answers, and its value. It used to increment
 * the counter twice on every boot to show it moved, which made it a count of
 * every boot, readable by anyone who can read the TPM: a record an amnesic
 * system must not leave. It now only reads. A counter defined and never
 * incremented cannot be read, so on that first boot alone it is incremented
 * once; after that no boot writes it.
 */
pub fn tpm_counter_selftest(bs: &BootServices) -> Result<u64, u32> {
    if !tpm_present(bs) {
        return Err(RC_NO_TPM);
    }
    nv_define_counter(bs)?;
    match nv_read_counter(bs) {
        Err(rc) if rc == RC_NV_UNINITIALIZED => {
            nv_increment(bs)?;
            nv_read_counter(bs)
        }
        other => other,
    }
}
