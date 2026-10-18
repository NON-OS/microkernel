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

//! A GPIO controller whose _AEI lists an edge pin with its own _E12, a
//! level pin with its own _L40, an edge pin with no method of its own, a
//! pin past 255, a pin listed twice and a GpioIo that is no event source;
//! every method records what ran. A second controller has no _EVT, a
//! third no _AEI.

use crate::enc::*;
use crate::tests::devices::serial::build::gpio;

fn template(parts: &[Vec<u8>]) -> Vec<u8> {
    let t = [parts.concat(), vec![0x79, 0]].concat();
    buffer(t.len() as u64, &t)
}

/// Each method stores its tag in \RAN and its argument, if any, in \ARG.
fn records(path: &str, args: u8, tag: u64) -> Vec<u8> {
    let mut body = store(&int(tag), &name("\\RAN"));
    if args == 1 {
        body.extend(store(&arg(0), &name("\\ARG")));
    }
    method(path, args, &body)
}

pub fn machine() -> Vec<u8> {
    let (edge, level_low, both_wake) = (0x01, 0x02, 0x01 | 0x04 | 0x10);
    let aei = template(&[
        gpio(true, edge, &[0x12, 0x20], "\\_SB.GPO0"),
        gpio(true, level_low, &[0x40, 0x1A5], "\\_SB.GPO0"),
        gpio(true, both_wake, &[0x12], "\\_SB.GPO0"),
        gpio(false, 0, &[0x50], "\\_SB.GPO0"),
    ]);
    let one = template(&[gpio(true, edge, &[0x300], "\\_SB.GPO1")]);
    cat(&[
        &name_obj("\\RAN", &int(0)),
        &name_obj("\\ARG", &int(0)),
        &device(
            "\\_SB.GPO0",
            &cat(&[
                &name_obj("_AEI", &aei),
                &records("_E12", 0, 1),
                &records("_L40", 0, 2),
                &records("_EVT", 1, 3),
            ]),
        ),
        &device("\\_SB.GPO1", &name_obj("_AEI", &one)),
        &device("\\_SB.GPO2", &name_obj("_UID", &int(2))),
    ])
}
