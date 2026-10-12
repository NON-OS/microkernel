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

//! A laptop's touchpad as firmware writes it: _STA decides which pad is
//! fitted and _CRS answers from that, its _DSM computes the descriptor
//! register, and _DSD carries properties; with its I2C and GPIO
//! controllers.

use super::build::{gpio, i2c, template};
use crate::enc::*;

pub fn machine() -> Vec<u8> {
    let fitted = template(&[
        i2c(0x2C, 400_000, false, "\\_SB.PCI0.I2C1"),
        gpio(true, 0x03, &[0x55], "\\_SB.GPO0"),
    ]);
    let other = template(&[i2c(0x15, 100_000, false, "\\_SB.PCI0.I2C1")]);
    let sta = cat(&[&store(&int(1), &name("\\TPDS")), &ret(&int(0x0F))]);
    let crs = cat(&[&if_(&name("\\TPDS"), &ret(&fitted)), &ret(&other)]);
    let one = e(0x93, &[&arg(2), &int(1)]);
    let dsm =
        cat(&[&if_(&one, &ret(&e(0x72, &[&name("\\BASE"), &int(0x20), &[0]]))), &ret(&int(0))]);
    let props = package(
        3,
        &[
            package(2, &[string("compatible"), string("vendor,pad")]),
            package(2, &[string("max-fingers"), int(5)]),
            package(
                2,
                &[string("reset-gpios"), package(4, &[name("\\_SB.GPO0"), int(0), int(1), int(0)])],
            ),
        ],
    );
    let dsd = package(
        4,
        &[
            buffer(16, &[0u8; 16]),
            package(0, &[]),
            buffer(16, &crate::serial::DEVICE_PROPERTIES_UUID),
            props,
        ],
    );
    let pad = cat(&[
        &name_obj("_HID", &string("ELAN0001")),
        &name_obj("_CID", &string("PNP0C50")),
        &method("_STA", 0, &sta),
        &method("_CRS", 0, &crs),
        &method("_DSM", 4, &dsm),
        &name_obj("_DSD", &dsd),
    ]);
    cat(&[
        &name_obj("\\TPDS", &int(0)),
        &name_obj("\\BASE", &int(0)),
        &device("\\_SB.GPO0", &name_obj("_UID", &int(1))),
        &device("\\_SB.PCI0", &device("I2C1", &device("TPD0", &pad))),
    ])
}
