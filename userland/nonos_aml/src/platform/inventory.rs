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

//! Every device the namespace holds, with its ids and status, for the log
//! and for the services that find their hardware through ACPI.

use super::eval::{child, integer, Eval};
use super::ids::{compatible_ids, hardware_id, HardwareId};
use super::sta::sta;
use super::walk::walk;
use crate::exec::Fault;
use crate::load::Aml;
use crate::namespace::{Id, Kind};
use alloc::vec::Vec;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeviceInfo {
    pub id: Id,
    pub hid: Option<HardwareId>,
    pub cids: Vec<HardwareId>,
    pub uid: Option<u64>,
    pub adr: Option<u64>,
    pub sta: u64,
}

/// Devices in definition order, depth first, and the objects that failed.
pub fn inventory(aml: &mut Aml, eval: &mut Eval<'_>) -> (Vec<DeviceInfo>, Vec<(Id, Fault)>) {
    let (mut out, mut failed) = (Vec::new(), Vec::new());
    walk(aml, &mut |aml, c| {
        let Some(kind) = aml.ns.get(c).map(|e| e.kind) else { return false };
        if kind == Kind::Device {
            out.push(describe(aml, eval, c, &mut failed));
        }
        kind.is_container()
    });
    (out, failed)
}

fn describe(
    aml: &mut Aml,
    eval: &mut Eval<'_>,
    c: Id,
    failed: &mut Vec<(Id, Fault)>,
) -> DeviceInfo {
    let mut get = |aml: &mut Aml, seg| match child(aml, eval, c, seg, Vec::new()) {
        Some(Ok(v)) => Some(v),
        Some(Err(f)) => {
            failed.push((c, f));
            None
        }
        None => None,
    };
    let hid = get(aml, *b"_HID").and_then(|v| hardware_id(&v));
    let cids = get(aml, *b"_CID").map_or_else(Vec::new, |v| compatible_ids(&v));
    let uid = get(aml, *b"_UID").and_then(|v| integer(&v));
    let adr = get(aml, *b"_ADR").and_then(|v| integer(&v));
    let sta = sta(aml, eval, c).unwrap_or_else(|f| {
        failed.push((c, f));
        0
    });
    DeviceInfo { id: c, hid, cids, uid, adr, sta }
}
