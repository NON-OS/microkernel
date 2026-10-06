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

//! The rollback floor over the firmware's TCG2 protocol.

use super::floor_seq::{floor_and_base_with, raise_with};
use crate::log::logger::log_info;
use crate::security::{audit, AuditEvent};
use crate::security::tpm_extend::{submit_tpm_command, tpm_present};
use uefi::table::boot::BootServices;

/// The TPM's rollback floor, `None` without a TPM or a readable counter.
/// A base set on this read is said and audited: on a machine that had one, it
/// means the owner deleted it, and the floor began again at 0.
pub fn read_floor(bs: &BootServices) -> Option<u64> {
    if !tpm_present(bs) {
        return None;
    }
    let (floor, set) =
        floor_and_base_with(|cmd: &[u8], resp: &mut [u8]| submit_tpm_command(bs, cmd, resp).ok())?;
    if set {
        log_info("rollback", "floor base set from the tpm counter: the floor begins at 0");
        audit(AuditEvent::PolicyEnforced, 0, b"rollback floor base set");
    }
    Some(floor)
}

/// Raise the floor to `target`; false without a TPM or when an increment fails.
pub fn commit_floor(bs: &BootServices, target: u64) -> bool {
    if !tpm_present(bs) {
        return false;
    }
    raise_with(|cmd: &[u8], resp: &mut [u8]| submit_tpm_command(bs, cmd, resp).ok(), target)
}
