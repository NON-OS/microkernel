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

extern crate alloc;

use crate::log::logger::{log_error, log_info};
use alloc::format;
use uefi::cstr16;
use uefi::prelude::*;

pub fn check_signature_db(st: &mut SystemTable<Boot>) -> bool {
    let rt = st.runtime_services();
    let mut buf = [0u8; 4096];
    /*
     * db lives under EFI_IMAGE_SECURITY_DATABASE_GUID, not the global
     * variable GUID that holds PK, KEK and SecureBoot. Asked under the
     * global GUID, every spec-conformant firmware answers NOT_FOUND.
     */
    match rt.get_variable(
        cstr16!("db"),
        &uefi::table::runtime::VariableVendor::IMAGE_SECURITY_DATABASE,
        &mut buf,
    ) {
        Ok(_) => {
            log_info("security", "Signature DB present");
            buf.iter().any(|&b| b != 0)
        }
        /*
         * A db larger than the probe buffer exists and is not empty; one
         * holding several vendors' certificates is past 4 KiB.
         */
        Err(e) if e.status() == Status::BUFFER_TOO_SMALL => {
            log_info("security", "Signature DB present, larger than the probe buffer");
            true
        }
        Err(e) => {
            log_error("security", &format!("Signature DB missing: {:?}", e.status()));
            false
        }
    }
}
