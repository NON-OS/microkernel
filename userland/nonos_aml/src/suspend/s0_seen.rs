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

//! S0 low-power idle as offered, read without running anything that acts:
//! the FADT's flag (its bit recalled, unverified), the known UUIDs a present
//! PNP0D80 device's _DSM body holds (never called), and whether \_S3 exists.

use super::vendor::{uuids, Uuids};
use crate::load::Aml;
use crate::namespace::{Id, Kind, Origin, ROOT};
use crate::platform::{DeviceInfo, PRESENT};

/// FADT flags bit 21: S0 low-power idle is as good as S3 here (5.2.9).
pub const LOW_POWER_S0_IDLE: u32 = 1 << 21;
const FLAGS_AT: usize = 112;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lps0Seen {
    /// No present PNP0D80 device.
    Absent,
    NoDsm,
    /// A _DSM, and the known UUIDs its body holds.
    Dsm(Uuids),
}

/// The flag from the FADT's bytes; false for a table too short to carry it.
pub fn fadt_s0_idle(fadt: &[u8]) -> bool {
    let f = fadt.get(FLAGS_AT..FLAGS_AT + 4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]));
    f.is_some_and(|f| f & LOW_POWER_S0_IDLE != 0)
}

/// Offered: the FADT says so, or an LPS0 _DSM holds a known UUID.
pub fn offered(flag: bool, lps0: Lps0Seen) -> bool {
    flag || matches!(lps0, Lps0Seen::Dsm(u) if u != Uuids::default())
}

pub fn has_s3(aml: &Aml) -> bool {
    aml.ns.child(ROOT, *b"_S3_").is_some()
}

pub fn lps0_seen(aml: &Aml, devices: &[DeviceInfo]) -> Lps0Seen {
    let lps0 = |d: &&DeviceInfo| d.hid.iter().chain(&d.cids).any(|h| h.bytes() == b"PNP0D80");
    let Some(dev) = devices.iter().filter(|d| d.sta & PRESENT != 0).find(lps0) else {
        return Lps0Seen::Absent;
    };
    match aml.ns.child(dev.id, *b"_DSM") {
        None => Lps0Seen::NoDsm,
        Some(dsm) => Lps0Seen::Dsm(body(aml, dsm).map(uuids).unwrap_or_default()),
    }
}

/// A Method's body bytes, as loaded; None for any other object.
fn body(aml: &Aml, id: Id) -> Option<&[u8]> {
    let e = aml.ns.get(id)?;
    match (e.kind, e.origin) {
        (Kind::Method(_), Origin::Table { table, node }) => {
            Some(aml.span_bytes(table, aml.kid(table, node, 2)))
        }
        _ => None,
    }
}
