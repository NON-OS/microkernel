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

//! MkAcpiSleep: the SLP_TYP values \_S5 gave, for the kernel's soft-off.

use super::acpi::{ok, tag4, Errno};
use nonos_aml::sleep::SleepTypes;
use nonos_libc::mk_syscall_raw;

const SLEEP: i64 = tag4(b"MASL");

pub const SOFT_OFF: u8 = 5;

pub fn acpi_sleep(state: u8, t: SleepTypes) -> Result<(), Errno> {
    let args = [u64::from(state), u64::from(t.a), u64::from(t.b), 0, 0, 0];
    ok(mk_syscall_raw(SLEEP, args)).map(|_| ())
}
