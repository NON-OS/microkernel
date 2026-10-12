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

//! What NONOS asks of firmware through _OSC. Only what it does: on each PCI
//! host bridge, extended configuration space (it reaches ECAM) and MSI (its
//! drivers use MSI-X) are supported, and every control is asked about but
//! only `osc_host::CLAIM` is taken, so native hot plug, PME, AER and the PCI
//! Express capability stay with firmware until a driver takes them on (PCI
//! Firmware 3.3, 4.5.1). The platform-wide _OSC is called with nothing
//! claimed, which tells firmware an operating system that knows _OSC is
//! present.

use super::eval::evaluator;
use crate::host::CapsuleHost;
use crate::sys::log::Line;
use alloc::vec::Vec;
use nonos_aml::load::Aml;
use nonos_aml::namespace::{Id, Kind, ROOT};
use nonos_aml::platform::{hardware_id, negotiate, HardwareId, PLATFORM_UUID};

fn is_host_bridge(aml: &mut Aml, host: &mut CapsuleHost, dev: Id) -> bool {
    let Some(hid) = aml.ns.child(dev, *b"_HID") else { return false };
    let id = super::eval::evaluate(aml, host, hid, Vec::new()).ok().and_then(|v| hardware_id(&v));
    matches!(id, Some(HardwareId::Eisa(ref b)) if b == b"PNP0A08" || b == b"PNP0A03")
}

pub fn negotiate_all(aml: &mut Aml, host: &mut CapsuleHost) {
    if let Some(osc) = aml.ns.child(ROOT, *b"_SB_").and_then(|sb| aml.ns.child(sb, *b"_OSC")) {
        let r = negotiate(aml, &mut evaluator(host), osc, (&PLATFORM_UUID, 1), &[0, 0]);
        Line::new(b"\\_SB._OSC ").str(if r.is_ok() { b"ok" } else { b"refused" }).end();
    }
    let devices: Vec<Id> = (0..aml.ns.len() as Id)
        .filter(|&i| aml.ns.get(i).is_some_and(|e| e.kind == Kind::Device))
        .collect();
    for dev in devices {
        let Some(osc) = aml.ns.child(dev, *b"_OSC") else { continue };
        if !is_host_bridge(aml, host, dev) {
            continue;
        }
        super::osc_host::host_bridge(aml, host, dev, osc);
    }
}
