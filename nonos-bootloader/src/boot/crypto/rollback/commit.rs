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

use uefi::prelude::*;

use crate::boot::util::fatal_reset;
use crate::image_format::{has_production_footer, parse_image_footer};
use crate::log::logger::{log_error, log_info};

use crate::menu::SecurityMode;
use crate::security::commit_floor;

pub fn commit_rollback(st: &mut SystemTable<Boot>, data: &[u8], mode: SecurityMode, gop: bool) {
    if !has_production_footer(data) {
        return;
    }
    let parsed = match parse_image_footer(data) {
        Ok(parsed) => parsed,
        Err(e) => {
            if mode.requires_signature() {
                log_error("rollback", "kernel version footer parse failed");
                fatal_reset(st, e.as_str());
            }
            return;
        }
    };
    // Raise the floor to the signed rollback_index, the authenticated field
    // the check gates on. image_version is unsigned and must not drive it.
    let rollback_index = parsed.footer.rollback_index as u64;
    if commit_floor(st.boot_services(), rollback_index) {
        log_info("rollback", "tpm rollback floor committed");
    } else {
        super::raise::raise_failed(st, mode, gop, rollback_index);
    }
}

