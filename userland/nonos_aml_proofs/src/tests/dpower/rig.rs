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

//! The devices the D-state proofs move: PRA (ResourceOrder 1) and PRB
//! (ResourceOrder 0) shared between them; DEV1 whose _PS0 faults while
//! \FAIL is set; DEV2 with a D2; DEV3 with no D1; DEV4 whose D3hot keeps
//! PRB; and each step said through Debug.

use crate::consts::at;
use crate::enc::*;
use crate::load::Aml;
use crate::probe::Probe;

pub fn say(n: u64) -> Vec<u8> {
    store(&int(n), &op(0x5B31))
}

fn resource(path: &str, order: u16, on: u64, off: u64) -> Vec<u8> {
    power_res(path, 0, order, &cat(&[&method("_ON_", 0, &say(on)), &method("_OFF", 0, &say(off))]))
}

fn list(seg: &str, names: &[&str]) -> Vec<u8> {
    name_obj(seg, &package(names.len() as u8, &names.iter().map(|n| name(n)).collect::<Vec<_>>()))
}

pub fn src() -> Vec<u8> {
    let divide = e(0x78, &[&int(1), &name("\\ZRO"), &[0], &[0]]);
    let ps0 = cat(&[&if_(&name("\\FAIL"), &divide), &say(0x40), &store(&int(0), &name("\\DST"))]);
    let ps3 = cat(&[&say(0x43), &store(&int(3), &name("\\DST"))]);
    let dev1 = cat(&[
        &list("_PR0", &["\\PRA", "\\PRB"]),
        &method("_PS0", 0, &ps0),
        &method("_PS3", 0, &ps3),
        &method("_PSC", 0, &ret(&name("\\DST"))),
    ]);
    cat(&[
        &name_obj("\\FAIL", &int(0)),
        &name_obj("\\ZRO", &int(0)),
        &name_obj("\\DST", &int(3)),
        &resource("\\PRA", 1, 0xA1, 0xA0),
        &resource("\\PRB", 0, 0xB1, 0xB0),
        &device("\\_SB.DEV1", &dev1),
        &device("\\_SB.DEV2", &cat(&[&list("_PR0", &["\\PRA"]), &list("_PR2", &["\\PRA"])])),
        &device("\\_SB.DEV3", &cat(&[&method("_PS0", 0, &[]), &method("_PS3", 0, &[])])),
        &device("\\_SB.DEV4", &cat(&[&list("_PR0", &["\\PRA"]), &list("_PR3", &["\\PRB"])])),
    ])
}

pub fn said(p: &Probe) -> Vec<String> {
    p.debug.clone()
}

pub fn ints(v: &[u64]) -> Vec<String> {
    v.iter().map(|n| format!("Integer({n})")).collect()
}

pub fn ids(aml: &Aml) -> [u32; 6] {
    ["\\_SB.DEV1", "\\_SB.DEV2", "\\_SB.DEV3", "\\_SB.DEV4", "\\PRA", "\\PRB"]
        .map(|p| at(aml, p).unwrap())
}
