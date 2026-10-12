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

//! The processors the cpu proofs read: a Processor object with _PSS, _PCT,
//! _PPC and _CST, one with nothing, an ACPI0007 device with _CPC, one whose
//! _STA says absent, and a device that is no processor.

use super::build::{cpc, cstate, ints, reg};
use crate::enc::*;

pub fn machine() -> Vec<u8> {
    let pss = package(
        3,
        &[
            ints(&[2400, 15000, 10, 10, 0x1800, 0x1800]),
            ints(&[1600, 9000, 10, 10, 0x1000, 0x1000]),
            ints(&[800, 4000, 10, 10, 0x800, 0x800]),
        ],
    );
    let cst = package(
        4,
        &[
            int(3),
            cstate(reg(0x7F, 1, 2, 1, 0), 1, 1, 1000),
            cstate(reg(1, 8, 0, 0, 0x414), 2, 50, 500),
            cstate(reg(1, 8, 0, 0, 0x415), 3, 150, 100),
        ],
    );
    let pct = package(2, &[reg(0x7F, 64, 0, 0, 0x199), reg(0x7F, 64, 0, 0, 0x198)]);
    let cpu0 = cat(&[
        &name_obj("_PCT", &pct),
        &name_obj("_PSS", &pss),
        &method("_PPC", 0, &ret(&int(1))),
        &name_obj("_CST", &cst),
    ]);
    let acpi0007 = |uid: u64, rest: &[u8]| {
        cat(&[&name_obj("_HID", &string("ACPI0007")), &name_obj("_UID", &int(uid)), rest])
    };
    cat(&[
        &processor("\\_PR.CPU0", 1, 0x410, 6, &cpu0),
        &processor("\\_PR.CPU1", 2, 0, 0, &[]),
        &device("\\_SB.CPU2", &acpi0007(7, &name_obj("_CPC", &cpc()))),
        &device("\\_SB.CPU3", &acpi0007(8, &name_obj("_STA", &int(0)))),
        &device("\\_SB.OTHR", &name_obj("_HID", &string("PNP0C0A"))),
    ])
}
