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

//! Reading the batteries and adapters through their methods: _STA for the
//! battery-present bit (ACPI 6.5, 6.3.7), _BIX or else _BIF, _BST, _PSR.

use super::state::{Battery, Power};
use crate::boot::evaluate;
use crate::host::CapsuleHost;
use alloc::vec::Vec;
use nonos_aml::load::Aml;
use nonos_aml::namespace::Id;
use nonos_aml::object::Object;
use nonos_aml::platform::{sta, DeviceInfo, HardwareId, BATTERY};
use nonos_aml::power::{online, parse_bif, parse_bix, parse_bst};

fn call(aml: &mut Aml, host: &mut CapsuleHost, dev: Id, seg: [u8; 4]) -> Option<Object> {
    let id = aml.ns.child(dev, seg)?;
    evaluate(aml, host, id, Vec::new()).ok()
}

/// Every PNP0C0A battery and ACPI0003 adapter in the inventory.
pub fn find(devices: &[DeviceInfo]) -> Power {
    let is = |d: &DeviceInfo, hid: &HardwareId| d.hid.as_ref() == Some(hid) || d.cids.contains(hid);
    let (bat, ac) = (HardwareId::Eisa(*b"PNP0C0A"), HardwareId::Text(b"ACPI0003".to_vec()));
    let batteries = devices.iter().filter(|d| is(d, &bat)).map(|d| Battery {
        id: d.id,
        present: false,
        info: None,
        status: None,
        told: 0,
    });
    let adapters = devices.iter().filter(|d| is(d, &ac)).map(|d| (d.id, None));
    Power { batteries: batteries.collect(), adapters: adapters.collect(), ..Power::default() }
}

pub fn read_info(b: &mut Battery, aml: &mut Aml, host: &mut CapsuleHost) {
    let mut eval = |aml: &mut Aml, id: Id, args| evaluate(aml, host, id, args);
    b.present = sta(aml, &mut eval, b.id).is_ok_and(|s| s & BATTERY != 0);
    if !b.present {
        (b.info, b.status) = (None, None);
        return;
    }
    let bix = call(aml, host, b.id, *b"_BIX").and_then(|v| parse_bix(&v));
    b.info = bix.or_else(|| call(aml, host, b.id, *b"_BIF").and_then(|v| parse_bif(&v)));
}

pub fn read_status(b: &mut Battery, aml: &mut Aml, host: &mut CapsuleHost) {
    if b.present {
        b.status = call(aml, host, b.id, *b"_BST").and_then(|v| parse_bst(&v));
        super::bst_told::tell(b);
    }
}

pub fn read_adapter(ac: &mut (Id, Option<bool>), aml: &mut Aml, host: &mut CapsuleHost) {
    ac.1 = call(aml, host, ac.0, *b"_PSR").and_then(|v| online(&v));
}
