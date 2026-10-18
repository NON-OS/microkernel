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

//! Finding the embedded controller (ACPI 6.5, 12.11): from the ECDT when
//! firmware has one, so EC fields work before the EC device's _REG; else
//! from the first PNP0C09 device's _CRS. Its _GPE says which event it
//! raises and _GLK whether its accesses take the Global Lock.

use super::state::Ec;
use crate::boot::evaluate;
use crate::host::CapsuleHost;
use crate::sys::acpi::{ports, table};
use alloc::vec::Vec;
use nonos_aml::ec::{ec_ports, parse_ecdt, query_methods};
use nonos_aml::load::Aml;
use nonos_aml::namespace::Id;
use nonos_aml::namespace::Kind;
use nonos_aml::object::Object;
use nonos_aml::platform::{hardware_id, integer, HardwareId};

fn eval_child(aml: &mut Aml, host: &mut CapsuleHost, dev: Id, seg: [u8; 4]) -> Option<Object> {
    let id = aml.ns.child(dev, seg)?;
    evaluate(aml, host, id, Vec::new()).ok()
}

fn devices(aml: &Aml) -> Vec<Id> {
    (0..aml.ns.len() as Id)
        .filter(|&i| aml.ns.get(i).is_some_and(|e| e.kind == Kind::Device))
        .collect()
}

/// The device the ECDT names, or the first whose _HID is PNP0C09.
fn device(aml: &mut Aml, host: &mut CapsuleHost, ecdt_path: Option<&[u8]>) -> Option<Id> {
    let all = devices(aml);
    if let Some(p) = ecdt_path {
        return all.into_iter().find(|id| crate::boot::path(aml, *id).as_bytes() == p);
    }
    let pnp0c09 = Some(HardwareId::Eisa(*b"PNP0C09"));
    all.into_iter()
        .find(|&d| eval_child(aml, host, d, *b"_HID").and_then(|v| hardware_id(&v)) == pnp0c09)
}

pub fn find(aml: &mut Aml, host: &mut CapsuleHost) -> Option<Ec> {
    let ecdt = table(*b"ECDT", 0).ok().and_then(|t| parse_ecdt(&t));
    let dev = device(aml, host, ecdt.as_ref().map(|e| e.path.as_slice()));
    let crs = || -> Option<_> {
        let Object::Buffer(b) = eval_child(aml, host, dev?, *b"_CRS")? else { return None };
        ec_ports(&b.clone().borrow())
    };
    let ports_found = ecdt.as_ref().map(|e| e.ports).or_else(crs)?;
    let ok =
        ports(ports_found.data as u64, 1).is_ok() && ports(ports_found.command as u64, 1).is_ok();
    if !ok {
        return None;
    }
    let gpe = dev.and_then(|d| eval_child(aml, host, d, *b"_GPE")).and_then(|v| integer(&v));
    let gpe = gpe.map(|g| g as u16).or(ecdt.as_ref().map(|e| e.gpe as u16));
    let glk =
        dev.and_then(|d| eval_child(aml, host, d, *b"_GLK")).and_then(|v| integer(&v)) == Some(1);
    let queries = dev.map(|d| query_methods(&aml.ns, d)).unwrap_or_default();
    Some(Ec { ports: ports_found, gpe, glk, queries })
}
