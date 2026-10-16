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

//! One PCI host bridge's _OSC: the query asks for every PCIe control (native
//! hot plug, SHPC, PME, AER, the capability structure, LTR, DPC), which
//! changes nothing and says what firmware would give up; the commit claims
//! only `CLAIM`. Both answers go to the kernel with the buses the bridge
//! decodes, so its AER and hot plug know what firmware still owns. Control
//! bits are recalled from the PCI Firmware specification: unverified.

use super::eval::evaluator;
use crate::host::CapsuleHost;
use crate::hotplug::native::osc_grant::{OscGrant, NATIVE_HOT_PLUG, PCIE_CAPABILITY};
use crate::sys::log::Line;
use crate::sys::osc::osc_grant;
use alloc::vec::Vec;
use nonos_aml::load::Aml;
use nonos_aml::namespace::Id;
use nonos_aml::object::Object;
use nonos_aml::platform::{bus_range, child, integer, offer_then_claim, PCI_HOST_UUID};

const SUPPORT_EXT_CONFIG: u32 = 1 << 0;
const SUPPORT_MSI: u32 = 1 << 4;
/// Native hot plug, SHPC, PME, AER, capability structure, LTR, DPC.
const ASK: u32 = 0b1011_1111;
/// The one switch for native PCIe hot plug (`hotplug::native`), off:
/// firmware keeps it until the owner turns it on. On, NONOS takes it and
/// the capability structure it needs (recalled, unverified).
pub const NATIVE_HOT_PLUG_ON: bool = false;
/// What NONOS takes; firmware keeps PME, AER, LTR and DPC.
pub const CLAIM: u32 = if NATIVE_HOT_PLUG_ON { NATIVE_HOT_PLUG | PCIE_CAPABILITY } else { 0 };

/// Firmware's commit, for the hot-plug ports below the bridge.
pub fn host_bridge(aml: &mut Aml, host: &mut CapsuleHost, dev: Id, osc: Id) -> Option<OscGrant> {
    let words = [0, SUPPORT_EXT_CONFIG | SUPPORT_MSI, ASK];
    let claim = [0, u32::MAX, CLAIM];
    let mut eval = evaluator(host);
    let r = offer_then_claim(aml, &mut eval, osc, (&PCI_HOST_UUID, 1), &words, &claim);
    let line = Line::new(b"host bridge _OSC ");
    let Ok((offered, granted)) = r else {
        line.str(b"refused").end();
        return None;
    };
    let line = line.str(b"support ").hex(granted[1].into()).str(b" offered ");
    let line = line.hex(offered[2].into()).str(b" control ").hex(granted[2].into());
    let seg = child(aml, &mut eval, dev, *b"_SEG", Vec::new()).and_then(|r| r.ok());
    let segment = seg.as_ref().and_then(integer).unwrap_or(0) as u16;
    let buses = match child(aml, &mut eval, dev, *b"_CRS", Vec::new()) {
        Some(Ok(Object::Buffer(b))) => bus_range(&b.borrow()),
        _ => None,
    };
    let Some(buses) = buses else {
        line.str(b", no bus range: not published").end();
        return None;
    };
    match osc_grant(segment, buses, offered[2], granted[2]) {
        Ok(()) => line.end(),
        Err(e) => line.str(b", refused ").dec(e.unsigned_abs()).end(),
    }
    let (first, last) = buses;
    Some(OscGrant { segment, first, last, offered: offered[2], granted: granted[2] })
}
