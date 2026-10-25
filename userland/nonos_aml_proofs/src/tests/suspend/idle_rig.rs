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

//! A Low Power S0 Idle device whose _DSM offers Microsoft's and Intel's
//! sets with the function masks a test gives.

use crate::enc::*;
use crate::rig::Rig;
use crate::suspend::{INTEL_UUID, MICROSOFT_UUID};

/// The _DSM answers function 0 for a UUID with `mask` and Zero otherwise.
pub fn pep(ms_mask: &[u8], intel_mask: &[u8]) -> Rig {
    let set = |uuid: &[u8; 16], mask: &[u8]| {
        let is = e(0x93, &[&arg(0), &buffer(16, uuid)]);
        let f0 = if_(&e(0x93, &[&arg(2), &int(0)]), &ret(&buffer(mask.len() as u64, mask)));
        if_(&is, &cat(&[&f0, &ret(&int(0))]))
    };
    let dsm = cat(&[
        &set(&MICROSOFT_UUID, ms_mask),
        &set(&INTEL_UUID, intel_mask),
        &ret(&buffer(1, &[0])),
    ]);
    let body = cat(&[&name_obj("_HID", &int(0x800D_D041)), &method("_DSM", 4, &dsm)]);
    Rig::new(&device("\\_SB.PEPD", &body)).connected()
}
