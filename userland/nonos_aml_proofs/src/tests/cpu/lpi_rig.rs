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

//! A processor hierarchy for the _LPI proofs: SYS0 holds clusters CLU0 and
//! CLU1 and a plain device GRP2; CLU0 holds C000 and C001, CLU1 (no _UID)
//! holds C100, GRP2 holds C200, and C300 sits in no container.

use super::build::reg;
use crate::enc::*;

#[allow(clippy::too_many_arguments)]
pub fn state(res: u64, lat: u64, flags: u64, parent: u64, entry: Vec<u8>, ctr: Vec<u8>) -> Vec<u8> {
    let rest = [int(0), int(0), int(parent), entry, ctr, int(0), string("S")];
    package(10, &[&[int(res), int(lat), int(flags)][..], &rest].concat())
}

pub fn lpi(level: u64, states: &[Vec<u8>]) -> Vec<u8> {
    let head = [int(0), int(level), int(states.len() as u64)];
    package(3 + states.len() as u8, &[&head[..], states].concat())
}

fn node(path: &str, hid: &str, uid: Option<u64>, lpi: &[u8]) -> Vec<u8> {
    let uid = uid.map(|u| name_obj("_UID", &int(u))).unwrap_or_default();
    device(path, &cat(&[&name_obj("_HID", &string(hid)), &uid, &name_obj("_LPI", lpi)]))
}

pub fn machine() -> Vec<u8> {
    let core = lpi(
        0,
        &[
            state(1, 1, 1, 0, reg(0x7F, 32, 0, 3, 0xFFFF_FFFF), int(0)),
            state(500, 100, 1, 2, reg(0x7F, 32, 0, 3, 0x4000_0003), reg(0, 64, 0, 4, 0x9000)),
        ],
    );
    let cluster = lpi(
        1,
        &[state(10, 5, 1, 0, int(0x10000), int(0)), state(2000, 300, 1, 1, int(0x20000), int(0))],
    );
    let system = lpi(2, &[state(10000, 1000, 1, 0, int(0x100_0000), int(0))]);
    let cpu = |path: &str, uid: u64| node(path, "ACPI0007", Some(uid), &core);
    cat(&[
        &node("\\_SB.SYS0", "ACPI0010", Some(0x100), &system),
        &node("\\_SB.SYS0.CLU0", "ACPI0010", Some(0x10), &cluster),
        &cpu("\\_SB.SYS0.CLU0.C000", 0),
        &cpu("\\_SB.SYS0.CLU0.C001", 1),
        &node("\\_SB.SYS0.CLU1", "ACPI0010", None, &cluster),
        &cpu("\\_SB.SYS0.CLU1.C100", 2),
        &device("\\_SB.SYS0.GRP2", &[]),
        &cpu("\\_SB.SYS0.GRP2.C200", 3),
        &cpu("\\_SB.C300", 4),
    ])
}
