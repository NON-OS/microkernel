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

//! MkAcpiSlot: a native hot-plug slot's four registers on a port the
//! kernel chose, and the kernel's answers about what is at and below it,
//! so this capsule never reads a bridge's bus numbers or capability list.

use super::acpi::{ok, tag4, Errno};
use crate::hotplug::native::slot_wire::SECONDARY_BUS;
use crate::hotplug::native::slot_wire::{pack, unpack, Bdf, BELOW_AT, DLL_REPORTING, PORT_AT};
use alloc::vec::Vec;
use nonos_libc::mk_syscall_raw;

const SLOT: i64 = tag4(b"MRSL");
/// Past this many, a port's list is cut; a slot holds one device's functions.
const MOST_BELOW: u64 = 256;

fn call(port: u64, which: u64, write: u64, value: u64) -> Result<u64, Errno> {
    ok(mk_syscall_raw(SLOT, [port, which, write, value, 0, 0]))
}

pub fn slot_read(f: Bdf, which: u64) -> Result<u32, Errno> {
    call(pack(f), which, 0, 0).map(|v| v as u32)
}

pub fn slot_write(f: Bdf, which: u64, value: u16) -> Result<(), Errno> {
    call(pack(f), which, 1, u64::from(value)).map(|_| ())
}

/// The `n`th port the kernel lets this capsule drive, and whether it
/// reports its data link layer state. Asking for 0 has the kernel walk
/// its enumeration afresh.
pub fn port_at(n: u64) -> Option<(Bdf, bool)> {
    let got = call(0, PORT_AT, 0, n).ok()?;
    Some((unpack(got & !0xFFF)?, got & DLL_REPORTING != 0))
}

/// Every function the kernel's table holds below `f`, gathered before any
/// departs so that a departure never shifts the list under the caller.
pub fn below_port(f: Bdf) -> Vec<Bdf> {
    let at = |n| call(pack(f), BELOW_AT, 0, n).ok().and_then(unpack);
    (0..MOST_BELOW).map_while(at).collect()
}

pub fn secondary_bus(f: Bdf) -> Option<u8> {
    call(pack(f), SECONDARY_BUS, 0, 0).ok().map(|b| b as u8)
}
