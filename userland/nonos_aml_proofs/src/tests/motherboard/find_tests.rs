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

//! A machine's motherboard resource devices: the system board by _HID, a
//! chipset block by _CID under a vendor _HID, one with an empty _CRS, one
//! whose _STA says absent, and a device that keeps ranges of its own and
//! is not a motherboard device at all.

use super::build::{fixed_io, io, mem32_fixed, template};
use crate::enc::*;
use crate::motherboard::{motherboard_ranges, Reserved, Space};
use crate::platform::inventory;
use crate::probe::Probe;
use crate::run::boot;
use crate::tests::platform::run::evaluator;

fn crs(parts: &[Vec<u8>]) -> Vec<u8> {
    let t = template(parts);
    name_obj("_CRS", &buffer(t.len() as u64, &t))
}

fn dev(path: &str, ids: &[Vec<u8>], parts: &[Vec<u8>]) -> Vec<u8> {
    device(path, &cat(&[&ids.concat(), &crs(parts)]))
}

fn machine() -> Vec<u8> {
    let hid = |v: u64| name_obj("_HID", &int(v));
    cat(&[
        &dev("\\_SB.SYSB", &[hid(0x010C_D041)], &[mem32_fixed(0xE000_0000, 0x1000_0000)]),
        &dev(
            "\\_SB.LDRC",
            &[name_obj("_HID", &string("INT3F0D")), name_obj("_CID", &int(0x020C_D041))],
            &[io(0x4E, 0x4E, 2), fixed_io(0x61, 1)],
        ),
        &dev("\\_SB.NONE", &[hid(0x020C_D041)], &[]),
        &dev(
            "\\_SB.GONE",
            &[hid(0x020C_D041), name_obj("_STA", &int(0))],
            &[mem32_fixed(0xFEC0_0000, 0x1000)],
        ),
        &dev("\\_SB.UAR1", &[hid(0x0105_D041)], &[io(0x3F8, 0x3F8, 8)]),
    ])
}

#[test]
fn present_motherboard_devices_by_hid_or_cid() {
    let (mut aml, _) = boot(&machine(), 2);
    let mut probe = Probe::default();
    let (devices, _) = inventory(&mut aml, &mut evaluator(&mut probe));
    let got = motherboard_ranges(&mut aml, &mut evaluator(&mut probe), &devices);
    let r = |space, base, len| Reserved { space, base, len };
    assert_eq!(
        got,
        [r(Space::Memory, 0xE000_0000, 0x1000_0000), r(Space::Io, 0x4E, 2), r(Space::Io, 0x61, 1),],
        "the absent device and the UART add nothing"
    );
}
