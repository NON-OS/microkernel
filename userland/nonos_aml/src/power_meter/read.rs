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

//! The present meters, by _HID or _CID, and each one read: _PMC, _PMM (the
//! measurement, milliwatts), _GAI (the averaging interval, milliseconds)
//! and _GHL (the hardware cap, milliwatts). A method that is absent, fails
//! or answers 0xFFFFFFFF reads as None. Nothing here writes.

use super::pmc::{parse_pmc, Pmc};
use crate::load::Aml;
use crate::namespace::{Id, Seg};
use crate::platform::{child, integer, DeviceInfo, Eval, PRESENT};
use alloc::vec::Vec;

pub const POWER_METER_HID: &[u8] = b"ACPI000D";
const UNKNOWN: u64 = 0xFFFF_FFFF;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Meter {
    pub pmc: Option<Pmc>,
    pub power_mw: Option<u64>,
    pub interval_ms: Option<u64>,
    pub cap_mw: Option<u64>,
}

pub fn meters(devices: &[DeviceInfo]) -> Vec<Id> {
    let ours = |d: &&DeviceInfo| {
        let mut ids = d.hid.iter().chain(&d.cids);
        d.sta & PRESENT != 0 && ids.any(|h| h.bytes() == POWER_METER_HID)
    };
    devices.iter().filter(ours).map(|d| d.id).collect()
}

pub fn read_meter(aml: &mut Aml, eval: &mut Eval<'_>, dev: Id) -> Meter {
    let mut get = |seg: Seg| {
        let o = child(aml, eval, dev, seg, Vec::new())?.ok()?;
        integer(&o).filter(|v| *v != UNKNOWN)
    };
    let (power_mw, interval_ms, cap_mw) = (get(*b"_PMM"), get(*b"_GAI"), get(*b"_GHL"));
    let pmc = child(aml, eval, dev, *b"_PMC", Vec::new())
        .and_then(|r| r.ok())
        .and_then(|o| parse_pmc(&o));
    Meter { pmc, power_mw, interval_ms, cap_mw }
}
