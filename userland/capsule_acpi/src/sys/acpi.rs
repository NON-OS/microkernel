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

//! The four ACPI calls (src/syscall/microkernel/acpi/), made raw: a firmware
//! table, an OperationRegion grant, a port, a configuration access.

use alloc::vec;
use alloc::vec::Vec;
use nonos_libc::mk_syscall_raw;

const fn tag4(t: &[u8; 4]) -> i64 {
    u32::from_le_bytes(*t) as i64
}

const TABLE: i64 = tag4(b"MTBL");
const REGION: i64 = tag4(b"MRGN");
const PORT: i64 = tag4(b"MRIO");
pub(super) const PCI: i64 = tag4(b"MRPC");
pub(super) const POWER: i64 = tag4(b"MAPW");
pub(super) const EVENT: i64 = tag4(b"MAEV");

/// What the kernel answered: a negative errno.
pub type Errno = i64;

pub(super) fn ok(r: i64) -> Result<u64, Errno> {
    if r < 0 {
        Err(r)
    } else {
        Ok(r as u64)
    }
}

/// The `instance`th table with `sig`, checked by the kernel.
pub fn table(sig: [u8; 4], instance: u32) -> Result<Vec<u8>, Errno> {
    let sig = u32::from_le_bytes(sig) as u64;
    let len = ok(mk_syscall_raw(TABLE, [sig, instance as u64, 0, 0, 0, 0]))? as usize;
    let mut buf = vec![0u8; len];
    let ptr = buf.as_mut_ptr() as u64;
    let got = ok(mk_syscall_raw(TABLE, [sig, instance as u64, ptr, len as u64, 0, 0]))?;
    buf.truncate(got as usize);
    Ok(buf)
}

/// A SystemMemory window: the address `base` landed at, and its grant.
pub fn map(base: u64, len: u64) -> Result<(u64, u64), Errno> {
    let mut out = [0u64; 2];
    ok(mk_syscall_raw(REGION, [0, base, len, out.as_mut_ptr() as u64, 0, 0]))?;
    Ok((out[0], out[1]))
}

/// A SystemIO window, for `port_read` and `port_write`.
pub fn ports(base: u64, len: u64) -> Result<(), Errno> {
    ok(mk_syscall_raw(REGION, [1, base, len, 0, 0, 0])).map(|_| ())
}

pub fn port_read(port: u64, width: u8) -> Result<u32, Errno> {
    ok(mk_syscall_raw(PORT, [port, width as u64, 0, 0, 0, 0])).map(|v| v as u32)
}

pub fn port_write(port: u64, width: u8, value: u32) -> Result<(), Errno> {
    ok(mk_syscall_raw(PORT, [port, width as u64, 1, value as u64, 0, 0])).map(|_| ())
}
