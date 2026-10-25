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

//! MkSmbiosTable: the SMBIOS entry point (0) or structure table (1), as
//! the kernel checked and copied them.

use super::acpi::{ok, tag4, Errno};
use alloc::vec;
use alloc::vec::Vec;
use nonos_libc::mk_syscall_raw;

const SMBIOS: i64 = tag4(b"MSMB");
pub const ENTRY_POINT: u64 = 0;
pub const STRUCTURES: u64 = 1;

pub fn smbios(which: u64) -> Result<Vec<u8>, Errno> {
    let len = ok(mk_syscall_raw(SMBIOS, [which, 0, 0, 0, 0, 0]))? as usize;
    let mut buf = vec![0u8; len];
    let got = ok(mk_syscall_raw(SMBIOS, [which, buf.as_mut_ptr() as u64, len as u64, 0, 0, 0]))?;
    buf.truncate(got as usize);
    Ok(buf)
}
