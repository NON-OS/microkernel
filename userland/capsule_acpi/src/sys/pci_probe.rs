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

//! MkPciProbeFunction: after a function's _PS0, ask the kernel to read that
//! one function and add it to the device table if it now answers.

use super::acpi::{ok, tag4, Errno};
use nonos_libc::mk_syscall_raw;

const PROBE: i64 = tag4(b"MPPF");

/// Segment 0 only; the device id the kernel gave it.
pub fn probe_function(bus: u8, dev: u8, func: u8) -> Result<u64, Errno> {
    let devfn = u64::from(dev) << 3 | u64::from(func);
    ok(mk_syscall_raw(PROBE, [u64::from(bus), devfn, 0, 0, 0, 0]))
}
