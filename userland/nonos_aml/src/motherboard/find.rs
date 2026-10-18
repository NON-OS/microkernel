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

//! Every present motherboard resource device and the ranges its evaluated
//! _CRS names, all in one list: which device keeps a range is nobody's
//! business but firmware's. A _CRS that fails or is empty adds nothing.

use super::fixed::{reserved, Reserved};
use crate::load::Aml;
use crate::object::Object;
use crate::platform::{child, DeviceInfo, Eval, PRESENT};
use alloc::vec::Vec;

/// System board (PNP0C01) and motherboard resources (PNP0C02), by _HID or
/// by _CID.
pub const MOTHERBOARD_IDS: [&[u8]; 2] = [b"PNP0C01", b"PNP0C02"];

pub fn motherboard_ranges(
    aml: &mut Aml,
    eval: &mut Eval<'_>,
    devices: &[DeviceInfo],
) -> Vec<Reserved> {
    let mut out = Vec::new();
    for d in devices.iter().filter(|d| d.sta & PRESENT != 0) {
        let mut ids = d.hid.iter().chain(&d.cids);
        if !ids.any(|h| MOTHERBOARD_IDS.contains(&h.bytes())) {
            continue;
        }
        if let Some(Ok(Object::Buffer(b))) = child(aml, eval, d.id, *b"_CRS", Vec::new()) {
            out.extend(reserved(&b.borrow()));
        }
    }
    out
}
