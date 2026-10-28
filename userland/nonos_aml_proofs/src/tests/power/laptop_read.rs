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

//! capsule_acpi's reading (src/power/read.rs) through the same calls: the
//! inventory's PNP0C0A and ACPI0003 devices, _STA's battery bit, _BIX,
//! _BST and _PSR, into the capsule's own `Power`; and the lid's _LID.

use super::state::{Battery, Power};
use crate::button::lid_open;
use crate::exec::{ExecLimits, Machine};
use crate::namespace::Id;
use crate::object::Object;
use crate::platform::{inventory, sta, HardwareId, BATTERY};
use crate::power::{online, parse_bix, parse_bst};
use crate::rig::Rig;

fn call(r: &mut Rig, dev: Id, seg: [u8; 4]) -> Option<Object> {
    let id = r.aml.ns.child(dev, seg)?;
    Machine::new(&mut r.aml, &mut r.probe, ExecLimits::DEFAULT)
        .evaluate(&mut r.aml, id, vec![])
        .ok()
}

fn devices(r: &mut Rig, hid: &HardwareId) -> Vec<Id> {
    let probe = &mut r.probe;
    let mut ev = |aml: &mut _, id, args| {
        Machine::new(aml, probe, ExecLimits::DEFAULT).evaluate(aml, id, args)
    };
    let (all, _) = inventory(&mut r.aml, &mut ev);
    all.iter()
        .filter(|d| d.hid.as_ref() == Some(hid) || d.cids.contains(hid))
        .map(|d| d.id)
        .collect()
}

pub fn read(r: &mut Rig) -> Power {
    let mut p = Power::default();
    for id in devices(r, &HardwareId::Eisa(*b"PNP0C0A")) {
        let probe = &mut r.probe;
        let mut ev = |aml: &mut _, id, args| {
            Machine::new(aml, probe, ExecLimits::DEFAULT).evaluate(aml, id, args)
        };
        let present = sta(&mut r.aml, &mut ev, id).is_ok_and(|s| s & BATTERY != 0);
        let info = present.then(|| call(r, id, *b"_BIX").and_then(|v| parse_bix(&v))).flatten();
        let status = present.then(|| call(r, id, *b"_BST").and_then(|v| parse_bst(&v))).flatten();
        p.batteries.push(Battery { id, present, info, status, told: 0 });
    }
    for id in devices(r, &HardwareId::Text(b"ACPI0003".to_vec())) {
        let on = call(r, id, *b"_PSR").and_then(|v| online(&v));
        p.adapters.push((id, on));
    }
    p
}

pub fn lid(r: &mut Rig) -> Option<bool> {
    let id = devices(r, &HardwareId::Eisa(*b"PNP0C0D"))[0];
    call(r, id, *b"_LID").and_then(|v| lid_open(&v))
}
