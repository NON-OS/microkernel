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

//! MkAcpiCpu: one processor's tables, as `cpu::record` lays them out, for
//! the kernel's frequency and idle control.

use super::acpi::{ok, tag4, Errno};
use nonos_libc::mk_syscall_raw;

const CPU: i64 = tag4(b"MACP");

pub fn acpi_cpu(record: &[u8]) -> Result<(), Errno> {
    let args = [record.as_ptr() as u64, record.len() as u64, 0, 0, 0, 0];
    ok(mk_syscall_raw(CPU, args)).map(|_| ())
}
