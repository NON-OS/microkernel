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

//! MkAcpiThermalLimit: the ceiling passive cooling allows one processor,
//! by its ACPI processor UID, in percent of full performance; 100 lifts it.

use super::acpi::{ok, tag4, Errno};
use nonos_libc::mk_syscall_raw;

const THERMAL_LIMIT: i64 = tag4(b"MATL");

pub fn acpi_thermal_limit(uid: u64, percent: u8) -> Result<(), Errno> {
    ok(mk_syscall_raw(THERMAL_LIMIT, [uid, u64::from(percent), 0, 0, 0, 0])).map(|_| ())
}
