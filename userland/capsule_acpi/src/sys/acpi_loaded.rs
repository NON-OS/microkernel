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

//! MkAcpiLoaded: a table AML loads at run time (Load from an OperationRegion
//! or a buffer, or LoadTable), handed to the kernel before it is loaded, so
//! the measured list covers it and the attested root moves with it.

use super::acpi::{ok, tag4, Errno};
use nonos_libc::mk_syscall_raw;

const LOADED: i64 = tag4(b"MALD");

pub fn acpi_loaded(bytes: &[u8]) -> Result<(), Errno> {
    ok(mk_syscall_raw(LOADED, [bytes.as_ptr() as u64, bytes.len() as u64, 0, 0, 0, 0])).map(|_| ())
}
