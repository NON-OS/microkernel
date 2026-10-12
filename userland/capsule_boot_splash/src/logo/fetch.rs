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

//! Asking the kernel for the logo. Every refusal (no logo, not the splash,
//! a file larger than the splash will hold) means a splash without it.

use alloc::vec;

use nonos_libc::{mk_boot_logo, BOOT_LOGO_FILE, BOOT_LOGO_RECORD, BOOT_LOGO_RECORD_LEN};

use super::{prepare::MAX_FILE, Shown};

/// The logo for a surface of `w` by `h`, or None.
pub fn fetch(w: u32, h: u32) -> Option<Shown> {
    let mut rec = [0u8; BOOT_LOGO_RECORD_LEN];
    if mk_boot_logo(BOOT_LOGO_RECORD, rec.as_mut_ptr(), rec.len()) != rec.len() as i64 {
        return None;
    }
    let len = mk_boot_logo(BOOT_LOGO_FILE, core::ptr::null_mut(), 0);
    if len <= 0 || len as usize > MAX_FILE {
        return None;
    }
    let mut file = vec![0u8; len as usize];
    if mk_boot_logo(BOOT_LOGO_FILE, file.as_mut_ptr(), file.len()) != len {
        return None;
    }
    // The splash holds no display query, so the firmware mode is worked out
    // from the logo's own margins (`place::guess_mode`).
    super::prepare(&rec, &file, None, (w, h)).ok()
}
