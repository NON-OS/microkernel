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

//! The legacy devices the namespace declares, each with the resources its
//! evaluated _CRS gives, and the 8042 verdict that keeps a PS/2 driver off
//! ports 0x60 and 0x64 on a machine that has no controller there.

use super::boot_arch::BootArch;
use super::io::{io_ranges, memory_ranges, Range};
use super::kind::{kind_of, LegacyKind};
use crate::load::Aml;
use crate::namespace::Id;
use crate::object::Object;
use crate::platform::{child, interrupt_entries, DeviceInfo, Eval, HardwareId, Interrupt, PRESENT};
use alloc::vec::Vec;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Legacy {
    pub id: Id,
    pub kind: LegacyKind,
    pub uid: Option<u64>,
    pub io: Vec<Range>,
    pub memory: Vec<Range>,
    pub irqs: Vec<Interrupt>,
}

/// Present devices only; a _CRS that fails leaves its device with no
/// resources rather than out of the list, so a driver can still see it.
pub fn legacy_devices(aml: &mut Aml, eval: &mut Eval<'_>, devices: &[DeviceInfo]) -> Vec<Legacy> {
    let mut out = Vec::new();
    for d in devices.iter().filter(|d| d.sta & PRESENT != 0) {
        let ids: Vec<HardwareId> = d.hid.iter().chain(&d.cids).cloned().collect();
        let Some(kind) = kind_of(&ids) else { continue };
        let crs = match child(aml, eval, d.id, *b"_CRS", Vec::new()) {
            Some(Ok(Object::Buffer(b))) => b.borrow().clone(),
            _ => Vec::new(),
        };
        let (io, memory, irqs) = (io_ranges(&crs), memory_ranges(&crs), interrupt_entries(&crs));
        out.push(Legacy { id: d.id, kind, uid: d.uid, io, memory, irqs });
    }
    out
}

/// An 8042 is there when the namespace declares its keyboard or mouse
/// port, or the FADT says so: some laptops clear the FADT bit over an
/// EC-emulated 8042 they still declare. Neither, and there is none.
pub fn has_8042(boot: Option<BootArch>, found: &[Legacy]) -> bool {
    let declared = found.iter().any(|l| matches!(l.kind, LegacyKind::Keyboard | LegacyKind::Mouse));
    declared || boot.is_some_and(|b| b.has_8042)
}
