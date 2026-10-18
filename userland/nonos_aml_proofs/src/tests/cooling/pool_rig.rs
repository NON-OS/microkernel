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

//! The AML the power-resource proofs run: three resources, one whose
//! _ON_ executes Fatal, and three fans that share them.

use crate::enc::*;

pub fn say(n: u64) -> Vec<u8> {
    store(&int(n), &op(0x5B31))
}

fn resource(path: &str, on: &[u8], off: u64) -> Vec<u8> {
    power_res(path, 0, 0, &cat(&[&method("_ON_", 0, on), &method("_OFF", 0, &say(off))]))
}

pub fn src() -> Vec<u8> {
    let fatal = cat(&[&op(0x5B32), &[1], &2u32.to_le_bytes(), &int(3)]);
    let pr0 = |names: &[&str]| {
        name_obj(
            "_PR0",
            &package(names.len() as u8, &names.iter().map(|n| name(n)).collect::<Vec<_>>()),
        )
    };
    cat(&[
        &resource("\\_SB.PR1", &say(0x11), 0x10),
        &resource("\\_SB.PR2", &say(0x21), 0x20),
        &resource("\\_SB.PR3", &fatal, 0x30),
        &device(
            "\\_SB.FAN0",
            &cat(&[
                &pr0(&["\\_SB.PR1", "\\_SB.PR2"]),
                &method("_PS0", 0, &say(0x40)),
                &method("_PS3", 0, &say(0x43)),
            ]),
        ),
        &device("\\_SB.FAN1", &pr0(&["\\_SB.PR1"])),
        &device("\\_SB.FAN2", &pr0(&["\\_SB.PR1", "\\_SB.PR3"])),
    ])
}
