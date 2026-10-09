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

//! The kernel image held to the TPM's monotonic rollback floor, by profile.

use uefi::prelude::*;

use crate::boot::util::fatal_reset;
use crate::display::{log_warn, show_error_screen};
use crate::log::logger::{log_error, log_info};
use crate::menu::SecurityMode;
use crate::security::{audit, floor_rule, read_floor, AuditEvent, Floor};
use alloc::format;

pub(super) fn enforce_floor(st: &mut SystemTable<Boot>, mode: SecurityMode, gop: bool, index: u64) {
    match floor_rule(read_floor(st.boot_services()), mode.requires_tpm()) {
        Floor::Held(floor) => {
            log_info("rollback", &format!("tpm floor {} index {}", floor, index));
            if index < floor && mode.requires_signature() {
                log_error("rollback", "rollback index below TPM monotonic floor");
                if gop {
                    let msg = format!("Rollback: tpm floor {} above image index {}", floor, index);
                    show_error_screen(msg.as_bytes());
                }
                fatal_reset(st, "rollback index below TPM floor");
            }
        }
        Floor::Refuse => {
            log_error("rollback", "this profile needs a TPM rollback floor; none could be read");
            if gop {
                let msg = format!(
                    "{} needs a TPM: its rollback floor keeps an older signed kernel from booting",
                    mode.label()
                );
                show_error_screen(msg.as_bytes());
            }
            fatal_reset(st, "profile requires a TPM rollback floor");
        }
        Floor::Unprotected => {
            log_info("rollback", "no TPM floor: an older signed kernel would boot");
            audit(AuditEvent::PolicyEnforced, 0, b"rollback floor off: no TPM counter");
            if gop {
                log_warn(b"No TPM: rollback protection is off");
            }
        }
    }
}
