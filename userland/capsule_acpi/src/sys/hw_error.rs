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

//! MkAcpiHwError: one decoded hardware error record, counted by severity on
//! the kernel's posture line.

use super::acpi::{ok, tag4, Errno};
use nonos_libc::mk_syscall_raw;

const HW_ERROR: i64 = tag4(b"MAHE");

pub fn hw_error(severity: u32) -> Result<(), Errno> {
    ok(mk_syscall_raw(HW_ERROR, [u64::from(severity), 0, 0, 0, 0, 0])).map(|_| ())
}
