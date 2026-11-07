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

//! capsule_acpi's reading, its own src/power/read.rs mounted: the
//! inventory's PNP0C0A and ACPI0003 devices, _STA's battery bit, _BIX,
//! _BST and _PSR, into the capsule's own `Power`; and the lid's _LID.

use super::read::{find, read_adapter, read_info, read_status};
use super::state::Power;
use crate::button::lid_open;
use crate::exec::{ExecLimits, Machine};
use crate::load::Aml;
use crate::namespace::Id;
use crate::object::Object;
use crate::platform::{inventory, DeviceInfo, HardwareId};
use crate::rig::Rig;

fn call(r: &mut Rig, dev: Id, seg: [u8; 4]) -> Option<Object> {
    let id = r.aml.ns.child(dev, seg)?;
    Machine::new(&mut r.aml, &mut r.probe, ExecLimits::DEFAULT)
        .evaluate(&mut r.aml, id, vec![])
        .ok()
}

fn devices(r: &mut Rig) -> Vec<DeviceInfo> {
    let probe = &mut r.probe;
    let mut ev = |aml: &mut Aml, id, args| {
        Machine::new(aml, probe, ExecLimits::DEFAULT).evaluate(aml, id, args)
    };
    inventory(&mut r.aml, &mut ev).0
}

pub fn read(r: &mut Rig) -> Power {
    let mut p = find(&devices(r));
    let probe = &mut r.probe;
    let mut ev = |aml: &mut Aml, id, args| {
        Machine::new(aml, probe, ExecLimits::DEFAULT).evaluate(aml, id, args)
    };
    for b in p.batteries.iter_mut() {
        let _ = read_info(b, &mut r.aml, &mut ev);
        read_status(b, &mut r.aml, &mut ev);
    }
    for a in p.adapters.iter_mut() {
        read_adapter(a, &mut r.aml, &mut ev);
    }
    p
}

pub fn lid(r: &mut Rig) -> Option<bool> {
    let lid = HardwareId::Eisa(*b"PNP0C0D");
    let id = devices(r).iter().find(|d| d.hid.as_ref() == Some(&lid)).map(|d| d.id);
    call(r, id.expect("the lid"), *b"_LID").and_then(|v| lid_open(&v))
}
