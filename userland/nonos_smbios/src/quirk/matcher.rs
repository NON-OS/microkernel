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

//! Matching. Every listed field must match; a field firmware left blank
//! matches nothing, so a quirk never fires on a machine that does not say
//! what it is; an empty pattern matches nothing, nor does a quirk with no
//! fields at all (one that would fire on every machine belongs in its
//! driver, not here).

use super::types::{Device, DeviceMatch, DmiMatch, Field, Match, Quirk};
use crate::identity::Identity;
use alloc::vec::Vec;

fn value(id: &Identity, f: Field) -> &str {
    match f {
        Field::Vendor => &id.vendor,
        Field::Product => &id.product,
        Field::Version => &id.version,
        Field::BoardVendor => &id.board_vendor,
        Field::Board => &id.board,
        Field::BiosVendor => &id.bios_vendor,
        Field::BiosVersion => &id.bios_version,
    }
}

fn field_matches(id: &Identity, m: &DmiMatch) -> bool {
    let v = value(id, m.field);
    match m.is {
        Match::Exact(p) => !v.is_empty() && !p.is_empty() && v == p,
        Match::Prefix(p) => !v.is_empty() && !p.is_empty() && v.starts_with(p),
    }
}

pub fn dmi_matches(id: &Identity, dmi: &[DmiMatch]) -> bool {
    !dmi.is_empty() && dmi.iter().all(|m| field_matches(id, m))
}

fn device_matches(q: &DeviceMatch, d: &Device<'_>) -> bool {
    match (q, d) {
        (DeviceMatch::Pci { vendor, device }, Device::Pci { vendor: v, device: dv }) => {
            vendor == v && device == dv
        }
        (DeviceMatch::Acpi { hid }, Device::Acpi { hid: h }) => {
            !hid.is_empty() && hid.as_bytes() == *h
        }
        _ => false,
    }
}

/// The quirks in `table` for this machine and this device, in table order.
pub fn lookup<'t>(table: &'t [Quirk], id: &Identity, dev: &Device<'_>) -> Vec<&'t Quirk> {
    let ours = |q: &&Quirk| device_matches(&q.device, dev) && dmi_matches(id, q.dmi);
    table.iter().filter(ours).collect()
}
