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

//! `MkBootCensus(out, len) -> bytes`: the boot census the kernel last said
//! on serial (src/hardware/boot_census), the same lines, `[CENSUS] <class>
//! ...` then the summary, for About and the hardware report on a machine
//! with no serial port. 0 bytes before the census is first said. Read-only;
//! AttestRead, as MkLogTail, whose lines these are. ERRNO_INVAL for a null
//! or empty buffer; ERRNO_FAULT for one the caller cannot be written to; a
//! buffer too small takes what fits, a whole line at a time.

extern crate alloc;

use super::errnos::{ERRNO_FAULT, ERRNO_INVAL};

pub fn sys_boot_census(out_ptr: u64, out_len: u64) -> i64 {
    if out_ptr == 0 || out_len == 0 {
        return ERRNO_INVAL;
    }
    let text = crate::hardware::boot_census::text();
    let fits = crate::hardware::boot_census::whole_lines(&text, out_len as usize);
    if fits == 0 {
        return 0;
    }
    if crate::usercopy::validate_user_write(out_ptr, fits).is_err() {
        return ERRNO_FAULT;
    }
    match crate::usercopy::write_user_bytes(out_ptr, &text[..fits]) {
        Ok(()) => fits as i64,
        Err(_) => ERRNO_FAULT,
    }
}
