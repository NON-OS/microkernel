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

//! The PCI host bridges (PNP0A08, PNP0A03 by `_HID` or `_CID`), each with
//! its segment (`_SEG`), the buses it decodes (`_BBN`, then the bus range
//! its `_CRS` gives, else `_BBN` alone) and what its `_DSM` says.

use super::state::Host;
use crate::boot::{evaluator, Acpi};
use alloc::vec::Vec;
use nonos_aml::load::Aml;
use nonos_aml::namespace::{Id, Seg};
use nonos_aml::object::Object;
use nonos_aml::pci_dsm::host_facts;
use nonos_aml::platform::{bus_range, child, integer, DeviceInfo, Eval};

fn int(aml: &mut Aml, eval: &mut Eval<'_>, id: Id, seg: Seg) -> Option<u64> {
    child(aml, eval, id, seg, Vec::new()).and_then(|r| r.ok()).as_ref().and_then(integer)
}

fn is_host(d: &DeviceInfo) -> bool {
    d.hid.iter().chain(&d.cids).any(|h| matches!(h.bytes(), b"PNP0A08" | b"PNP0A03"))
}

pub fn hosts(acpi: &mut Acpi) -> Vec<Host> {
    let ids: Vec<_> = acpi.devices.iter().filter(|d| is_host(d)).map(|d| d.id).collect();
    let mut out = Vec::new();
    for id in ids {
        let (aml, eval) = (&mut acpi.aml, &mut evaluator(&mut acpi.host));
        let segment = int(aml, eval, id, *b"_SEG").unwrap_or(0) as u16;
        let bbn = int(aml, eval, id, *b"_BBN").unwrap_or(0) as u8;
        let crs = match child(aml, eval, id, *b"_CRS", Vec::new()).and_then(|r| r.ok()) {
            Some(Object::Buffer(b)) => bus_range(&b.borrow()),
            _ => None,
        };
        let facts = host_facts(aml, eval, id);
        out.push(Host { id, segment, buses: crs.unwrap_or((bbn, bbn)), facts });
    }
    out
}
