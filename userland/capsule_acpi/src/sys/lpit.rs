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

//! MkAcpiLpit: LPIT's low-power idle states, one field at a time, and
//! each one's residency counter now (`src/syscall/microkernel/acpi/lpit.rs`).

use super::acpi::{ok, tag4, Errno};
use nonos_libc::mk_syscall_raw;

const LPIT: i64 = tag4(b"MALP");

/// Fields of a state.
pub const ID_FLAGS: u64 = 0;
pub const MIN_RESIDENCY: u64 = 1;
pub const LATENCY: u64 = 2;
pub const FREQUENCY: u64 = 3;
pub const COUNTER: u64 = 4;

pub fn lpit(index: u32, field: u64) -> Result<u64, Errno> {
    ok(mk_syscall_raw(LPIT, [u64::from(index), field, 0, 0, 0, 0]))
}
