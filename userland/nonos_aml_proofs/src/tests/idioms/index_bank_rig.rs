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

//! CMOS-style registers behind an index/data pair, and two banks behind
//! one window chosen by a bank-select register.

use crate::enc::*;
use crate::probe_regs::Register;
use crate::rig::Rig;

pub fn rig() -> Rig {
    let index = pkg(
        0x5B86,
        &cat(&[
            &name("IDX"),
            &name("DAT"),
            &[0x01],
            &skip(0x10 * 8),
            &unit("CMR0", 8),
            &unit("CMR1", 16),
        ]),
    );
    let bank = |value: u64, unit_name: &str| {
        pkg(0x5B87, &cat(&[&name("BKR"), &name("BNKS"), &int(value), &[0x01], &unit(unit_name, 8)]))
    };
    let defs = cat(&[
        &region("IOR", 1, 0x70, 2),
        &field("IOR", 0x01, &[unit("IDX", 8), unit("DAT", 8)]),
        &index,
        &region("BKR", 1, 0x80, 1),
        &region("BSEL", 1, 0x81, 1),
        &field("BSEL", 0x01, &[unit("BNKS", 8)]),
        &bank(1, "B1F0"),
        &bank(2, "B2F0"),
        &method("RC1", 0, &ret(&name("CMR1"))),
        &method("RC0", 0, &ret(&name("CMR0"))),
        &method("WC1", 0, &cat(&[&store(&int(0x5566), &name("CMR1")), &ret(&name("CMR1"))])),
        &method("RB1", 0, &ret(&name("B1F0"))),
        &method("RB2", 0, &ret(&name("B2F0"))),
    ]);
    let mut r = Rig::new(&defs).connected();
    let cmos = [(0x10, 0xAA), (0x11, 0x34), (0x12, 0x12)].into_iter().collect();
    r.probe.regs.insert((1, 0x71), Register::Data { index: (1, 0x70), bank: cmos });
    let banks = [(1, 0x11), (2, 0x22)].into_iter().collect();
    r.probe.regs.insert((1, 0x80), Register::Data { index: (1, 0x81), bank: banks });
    r
}
