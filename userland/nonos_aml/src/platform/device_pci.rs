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

//! The PCI path to a device the namespace describes: its _ADR and every
//! ancestor's, up to the PCI host bridge (PNP0A03 or PNP0A08) and its _SEG
//! and _BBN (ACPI 6.5, 6.1.1, 6.5.5, 6.5.6). `function_of` then turns it
//! into a bus, device and function.

use super::eval::{child, integer, Eval};
use super::ids::{compatible_ids, hardware_id};
use crate::exec::PciPath;
use crate::load::Aml;
use crate::namespace::{Id, Kind, Seg, ROOT};
use alloc::vec::Vec;

fn int(aml: &mut Aml, eval: &mut Eval<'_>, at: Id, seg: Seg) -> Option<u64> {
    child(aml, eval, at, seg, Vec::new())?.ok().as_ref().and_then(integer)
}

fn is_host_bridge(aml: &mut Aml, eval: &mut Eval<'_>, at: Id) -> bool {
    let hid = child(aml, eval, at, *b"_HID", Vec::new()).and_then(|r| r.ok());
    let cid = child(aml, eval, at, *b"_CID", Vec::new()).and_then(|r| r.ok());
    let mut ids: Vec<_> = hid.as_ref().and_then(hardware_id).into_iter().collect();
    ids.extend(cid.as_ref().map(compatible_ids).unwrap_or_default());
    ids.iter().any(|h| matches!(h.bytes(), b"PNP0A03" | b"PNP0A08"))
}

/// None when the device has no _ADR or no host bridge is above it.
pub fn device_pci_path(aml: &mut Aml, eval: &mut Eval<'_>, dev: Id) -> Option<PciPath> {
    int(aml, eval, dev, *b"_ADR")?;
    let (mut adrs, mut at) = (Vec::new(), dev);
    for _ in 0..64 {
        let e = *aml.ns.get(at)?;
        if at == ROOT {
            return None;
        }
        if e.kind == Kind::Device && is_host_bridge(aml, eval, at) {
            adrs.reverse();
            let segment = int(aml, eval, at, *b"_SEG").unwrap_or(0) as u16;
            let bus = int(aml, eval, at, *b"_BBN").unwrap_or(0) as u8;
            return Some(PciPath { segment, bus, adrs });
        }
        if let Some(adr) = int(aml, eval, at, *b"_ADR") {
            adrs.push(adr as u32);
        }
        at = e.parent;
    }
    None
}
