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

//! SMBIOS read once at start: the entry point and table the kernel copied,
//! walked by `nonos_smbios`, kept as the machine's identity for the port's
//! SMBIOS op and logged on one `[SMBIOS]` line. A machine without SMBIOS
//! says so and serves ENOENT.

use crate::sys::log::Line;
use crate::sys::smbios::{smbios, ENTRY_POINT, STRUCTURES};
use nonos_smbios::entry::parse_entry;
use nonos_smbios::identity::{identity, Identity};
use nonos_smbios::walk::structures;

pub fn read() -> Option<Identity> {
    let entry = smbios(ENTRY_POINT).ok().and_then(|e| parse_entry(&e));
    let Some(e) = entry else {
        Line::tagged(b"[SMBIOS] ").str(b"none").end();
        return None;
    };
    let table = smbios(STRUCTURES).ok()?;
    let found = structures(&table, e.count);
    let id = identity(&found);
    let l = Line::tagged(b"[SMBIOS] ").dec(u64::from(e.major)).str(b".").dec(u64::from(e.minor));
    let l = l.str(b" ").str(id.vendor.as_bytes()).str(b" ").str(id.product.as_bytes());
    let l =
        l.str(b", BIOS ").str(id.bios_version.as_bytes()).str(b" (").str(id.bios_date.as_bytes());
    l.str(b"), ").str(id.chassis.as_bytes()).str(b", structures ").dec(found.len() as u64).end();
    Some(id)
}
