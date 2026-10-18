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

//! The processors (ACPI 6.5, 8.4): Processor objects, deprecated but still
//! what most x86 firmware defines, and ACPI0007 devices. Each carries the
//! id the MADT's processor entries name it by: a Processor object's ProcID
//! byte, an ACPI0007 device's _UID.

use crate::load::Aml;
use crate::namespace::{Id, Kind, Origin};
use crate::platform::{child, hardware_id, integer, scan, sta, Eval, PRESENT};
use alloc::vec::Vec;

pub const PROCESSOR_HID: &[u8] = b"ACPI0007";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Processor {
    pub id: Id,
    pub uid: u64,
}

/// DefProcessor's operands are its name, then ProcID (19.6.109).
fn proc_id(aml: &Aml, id: Id) -> Option<u64> {
    match aml.ns.get(id)?.origin {
        Origin::Table { table, node } => Some(aml.imm(table, aml.kid(table, node, 1))),
        Origin::Predefined(_) => None,
    }
}

fn device_uid(aml: &mut Aml, eval: &mut Eval<'_>, id: Id) -> Option<u64> {
    let hid = child(aml, eval, id, *b"_HID", Vec::new())?.ok()?;
    if hardware_id(&hid)?.bytes() != PROCESSOR_HID {
        return None;
    }
    integer(&child(aml, eval, id, *b"_UID", Vec::new())?.ok()?)
}

/// In definition order, leaving out any whose _STA says absent.
pub fn processors(aml: &mut Aml, eval: &mut Eval<'_>) -> Vec<Processor> {
    let candidates = scan(aml, &|aml, id| {
        aml.ns.get(id).is_some_and(|e| matches!(e.kind, Kind::Processor | Kind::Device))
    });
    let mut out = Vec::new();
    for id in candidates {
        let uid = match aml.ns.get(id).map(|e| e.kind) {
            Some(Kind::Processor) => proc_id(aml, id),
            _ => device_uid(aml, eval, id),
        };
        let Some(uid) = uid else { continue };
        if sta(aml, eval, id).is_ok_and(|s| s & PRESENT != 0) {
            out.push(Processor { id, uid });
        }
    }
    out
}
