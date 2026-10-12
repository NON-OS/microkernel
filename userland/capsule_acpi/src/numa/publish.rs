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

//! Present root bridges (PNP0A03, PNP0A08 by _HID or _CID) with a _PXM of
//! their own or an ancestor's, and a bus-number range in their _CRS. A
//! bridge with no _PXM is left to the kernel's "unknown"; one with no bus
//! range is said and left, rather than guessed at from _BBN.

use crate::boot::{evaluator, path, Acpi};
use crate::sys::log::Line;
use crate::sys::numa::proximity;
use alloc::vec::Vec;
use nonos_aml::namespace::Id;
use nonos_aml::object::Object;
use nonos_aml::platform::{bus_range, child, integer, pxm, PRESENT};

const ROOT_BRIDGES: [&[u8]; 2] = [b"PNP0A03", b"PNP0A08"];

fn bridges(acpi: &Acpi) -> Vec<Id> {
    let root = |d: &&nonos_aml::platform::DeviceInfo| {
        let mut ids = d.hid.iter().chain(&d.cids);
        d.sta & PRESENT != 0 && ids.any(|h| ROOT_BRIDGES.contains(&h.bytes()))
    };
    acpi.devices.iter().filter(root).map(|d| d.id).collect()
}

fn one(acpi: &mut Acpi, dev: Id) {
    let Acpi { aml, host, .. } = acpi;
    let mut eval = evaluator(host);
    let Some(domain) = pxm(aml, &mut eval, dev) else { return };
    let seg = child(aml, &mut eval, dev, *b"_SEG", Vec::new()).and_then(|r| r.ok());
    let segment = seg.as_ref().and_then(integer).unwrap_or(0) as u16;
    let crs = match child(aml, &mut eval, dev, *b"_CRS", Vec::new()) {
        Some(Ok(Object::Buffer(b))) => bus_range(&b.borrow()),
        _ => None,
    };
    drop(eval);
    let line =
        Line::new(b"numa ").str(path(aml, dev).as_bytes()).str(b": domain ").dec(domain.into());
    let Some((first, last)) = crs else { return line.str(b", no bus range").end() };
    let line = line.str(b", buses ").hex(first.into()).str(b"-").hex(last.into());
    match proximity(segment, first, last, domain) {
        Ok(()) => line.end(),
        Err(e) => line.str(b", refused ").dec(e.unsigned_abs()).end(),
    }
}

pub fn publish(acpi: &mut Acpi) {
    for dev in bridges(acpi) {
        one(acpi, dev);
    }
}
