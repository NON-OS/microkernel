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

//! The ACPI namespace devices DMAR declared, said once at bring-up with the
//! requester id each one's DMA arrives under. A device listed without one
//! (no scope, or a path through bridges) has no context entry and is denied,
//! and the line says so rather than leave the operator to find out from a
//! dead touchpad or disk.

use crate::arch::x86_64::acpi::parser::other::{
    namespace_devices, namespace_dropped, namespace_requester,
};
use crate::sys::serial::Line;

pub(super) fn say() {
    for device in namespace_devices().iter() {
        let mut line = Line::new();
        line.str(b"[VT-D] ACPI device ").str(device.name());
        match namespace_requester(device.name()) {
            Some((unit, (bus, dev, func))) => {
                line.str(b" requester=").hex(bus as u64).str(b":").hex(dev as u64);
                line.str(b".").hex(func as u64).str(b" unit=").hex(unit as u64);
            }
            None => {
                line.str(b" has no one-hop scope; its DMA is denied");
            }
        }
        line.end();
    }
    let dropped = namespace_dropped();
    if dropped > 0 {
        let mut line = Line::new();
        line.str(b"[VT-D] WARNING ACPI device entries left out=").hex(dropped as u64);
        line.str(b"; malformed or past the room kept, their DMA is denied").end();
    }
}
