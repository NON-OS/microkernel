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

//! Asking the kernel for its verdict on the bootloader.

use super::record::{parse_boot_attest, BootAttest, BOOT_ATTEST_LEN};
use crate::syscall::{call_raw, N_MK_BOOT_ATTEST};

/// Fill `out` with the `MkBootAttest` record. Needs no capability: nothing in
/// it identifies the machine. The record's length, or a negative errno.
pub fn mk_boot_attest(out: &mut [u8; BOOT_ATTEST_LEN]) -> i64 {
    call_raw(N_MK_BOOT_ATTEST, [out.as_mut_ptr() as u64, out.len() as u64, 0, 0, 0, 0])
}

/// The verdict, read and parsed. `None` when the kernel did not answer or the
/// record did not parse, which a caller shows as unknown and never as a pass.
pub fn boot_attest() -> Option<BootAttest> {
    let mut buf = [0u8; BOOT_ATTEST_LEN];
    if mk_boot_attest(&mut buf) != BOOT_ATTEST_LEN as i64 {
        return None;
    }
    parse_boot_attest(&buf)
}
