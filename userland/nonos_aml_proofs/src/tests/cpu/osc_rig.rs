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

//! A processor whose _PSS appears only once _OSC has been committed.

use super::build::ints;
use crate::enc::*;

/// _OSC keeps the capability word it was passed in \CAP and, on the commit
/// (query bit clear), sets \LDED, as firmware does when it loads its CPU
/// SSDT; _PSS has states only once that happened.
pub fn machine(uuid_byte0: u8) -> Vec<u8> {
    let field = |i: u64, n: &str| e(0x8A, &[&arg(3), &int(i), &name(n)]);
    let byte0 = e(0x83, &[&e(0x88, &[&arg(0), &int(0), &[0]])]);
    let commit = e(0x93, &[&e(0x7B, &[&name("STS0"), &int(1), &[0]]), &int(0)]);
    let osc = cat(&[
        &field(0, "STS0"),
        &field(4, "CAP0"),
        &store(&name("CAP0"), &name("\\CAP")),
        &if_(
            &e(0x93, &[&byte0, &int(uuid_byte0 as u64)]),
            &if_(&commit, &store(&int(1), &name("\\LDED"))),
        ),
        &else_(&e(0x7D, &[&name("STS0"), &int(4), &name("STS0")])),
        &ret(&arg(3)),
    ]);
    let pss = cat(&[
        &if_(&name("\\LDED"), &ret(&package(1, &[ints(&[1800, 1, 1, 1, 0, 0])]))),
        &ret(&package(0, &[])),
    ]);
    cat(&[
        &name_obj("\\CAP", &int(0xFF)),
        &name_obj("\\LDED", &int(0)),
        &processor(
            "\\_PR.CPU0",
            0,
            0,
            0,
            &cat(&[&method("_OSC", 4, &osc), &method("_PSS", 0, &pss)]),
        ),
        &processor("\\_PR.CPU1", 1, 0, 0, &[]),
    ])
}
