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

//! MkAcpiHolds: whether a PCI function is claimed, as the capsule's own
//! look (mode 0, any holder) for the end-of-boot settle, or by one pid
//! (mode 1) for HOLDER.

use super::acpi::{ok, tag4, Errno};
use nonos_libc::mk_syscall_raw;

const HOLDS: i64 = tag4(b"MAHO");
const LOOK: u64 = 0;
const BY_PID: u64 = 1;

pub fn claimed_by_anyone((seg, bus, dev, func): (u16, u8, u8, u8)) -> Result<bool, Errno> {
    let bus_devfn = u64::from(bus) << 8 | u64::from(dev) << 3 | u64::from(func);
    ok(mk_syscall_raw(HOLDS, [LOOK, 0, u64::from(seg), bus_devfn, 0, 0])).map(|r| r == 1)
}

pub fn claimed_by(pid: u32, (seg, bus, dev, func): (u16, u8, u8, u8)) -> Result<bool, Errno> {
    let bus_devfn = u64::from(bus) << 8 | u64::from(dev) << 3 | u64::from(func);
    ok(mk_syscall_raw(HOLDS, [BY_PID, u64::from(pid), u64::from(seg), bus_devfn, 0, 0]))
        .map(|r| r == 1)
}
