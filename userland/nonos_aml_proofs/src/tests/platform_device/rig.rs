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

//! Platform controllers as firmware writes them: an AMD I2C host, a GPIO
//! controller with two windows, an eMMC host found by its _CID behind a
//! QWord window, one present but not enabled, one with no _CRS, one with
//! no memory window, and one whose id is not on the list.

use super::descriptors::{ext_irq, mem32, qword, template};
use crate::enc::*;

fn crs(parts: &[Vec<u8>]) -> Vec<u8> {
    let t = template(parts);
    name_obj("_CRS", &buffer(t.len() as u64, &t))
}

fn dev(path: &str, hid: &str, rest: &[Vec<u8>]) -> Vec<u8> {
    let body: Vec<Vec<u8>> = [vec![name_obj("_HID", &string(hid))], rest.to_vec()].concat();
    device(path, &body.concat())
}

pub fn machine() -> Vec<u8> {
    let level_low_shared = 0b0_1101;
    cat(&[
        &dev(
            "\\_SB.I2CA",
            "AMDI0010",
            &[crs(&[ext_irq(level_low_shared, 10), mem32(0xFEDC_2000, 0x1000)])],
        ),
        &dev(
            "\\_SB.GPO0",
            "INT33FC",
            &[crs(&[mem32(0xFED0_C000, 0x1000), mem32(0xFED0_D000, 0x1000), ext_irq(1, 49)])],
        ),
        &dev(
            "\\_SB.SDHA",
            "XYZ0001",
            &[
                name_obj("_CID", &string("80860F14")),
                name_obj("_UID", &int(3)),
                crs(&[qword(0x1_0000_0000, 0, 0x1000)]),
            ],
        ),
        &dev(
            "\\_SB.I2CB",
            "INT3432",
            &[name_obj("_STA", &int(0x0D)), crs(&[mem32(0xFE10_4000, 0x1000)])],
        ),
        &dev("\\_SB.I2CC", "INT33C2", &[]),
        &dev("\\_SB.I2CD", "INT33C3", &[crs(&[ext_irq(1, 7)])]),
        &dev("\\_SB.UNKN", "ABCD0001", &[crs(&[mem32(0xFE00_0000, 0x1000)])]),
    ])
}
