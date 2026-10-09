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

//! A rollback floor that could not be raised, by profile (REVIEW R20).

use uefi::prelude::*;

use crate::boot::util::fatal_reset;
use crate::display::{log_warn as panel_warn, show_error_screen};
use crate::log::logger::{log_error, log_warn};
use crate::menu::SecurityMode;
use crate::security::tpm_present;
use alloc::format;

/*
 * The floor stayed below this kernel's index, so the next boot would still
 * admit what this one refused. Hardened and Air-Gapped stop here; elsewhere
 * the screen says so. Without a TPM the check already said rollback is off.
 */
pub(super) fn raise_failed(st: &mut SystemTable<Boot>, mode: SecurityMode, gop: bool, index: u64) {
    if mode.requires_tpm() {
        log_error("rollback", "tpm rollback floor could not be raised");
        if gop {
            let msg = format!("Rollback floor could not be raised to index {}", index);
            show_error_screen(msg.as_bytes());
        }
        fatal_reset(st, "tpm rollback floor not raised");
    }
    if tpm_present(st.boot_services()) {
        log_warn("rollback", "tpm rollback floor could not be raised");
        if gop {
            panel_warn(b"Rollback floor not raised: an older kernel may still boot");
        }
    }
}
