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

//! A power meter as firmware writes one: _PMC listing measurement and an
//! enforced, configurable cap from 10 W to `max_cap` mW, reading 12.5 W
//! averaged over a second against a 45 W cap; _SHL storing what it is
//! given in \SHLV and refusing above 60 W. A second meter claims the id by
//! _CID with no methods; a third by _HID is absent.

use crate::enc::*;

pub fn pmc(caps: u64, configurable: u64, max_cap: u64) -> Vec<u8> {
    let ints = [caps, 0, 0, 95_000, 1000, 500, 5000, 100, configurable, 10_000, max_cap];
    let mut e: Vec<Vec<u8>> = ints.iter().map(|v| int(*v)).collect();
    e.extend([string("PM-1"), string("0042"), string("OEM")]);
    package(14, &e)
}

pub fn machine(pmc_body: &[u8]) -> Vec<u8> {
    let over = e(0x94, &[&arg(0), &int(60_000)]);
    let shl = cat(&[&store(&arg(0), &name("\\SHLV")), &if_(&over, &ret(&int(1))), &ret(&int(0))]);
    let meter = cat(&[
        &name_obj("_HID", &string("ACPI000D")),
        &name_obj("_PMC", pmc_body),
        &method("_PMM", 0, &ret(&int(12_500))),
        &method("_GAI", 0, &ret(&int(1000))),
        &method("_GHL", 0, &ret(&int(45_000))),
        &method("_SHL", 1, &shl),
    ]);
    let absent = cat(&[&name_obj("_HID", &string("ACPI000D")), &name_obj("_STA", &int(0))]);
    cat(&[
        &name_obj("\\SHLV", &int(0)),
        &method("\\GSHL", 0, &ret(&name("\\SHLV"))),
        &device("\\_SB.PMT0", &meter),
        &device("\\_SB.PMT1", &name_obj("_CID", &string("ACPI000D"))),
        &device("\\_SB.PMT2", &absent),
    ])
}
