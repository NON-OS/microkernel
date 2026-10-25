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

//! A host bridge whose `_DSM` answers function 0 only at revision 2 and
//! offers 5, 6 and 9 (6 must never run: it sets `\F6RN`); a GPU whose
//! label is well formed; a NIC whose label is not; a device with no `_DSM`.

use crate::enc::*;

fn is(n: u64, body: &[u8]) -> Vec<u8> {
    if_(&e(0x93, &[&arg(2), &int(n)]), body)
}

fn dsm(path: &str, mask: &[u8], rest: &[Vec<u8>]) -> Vec<u8> {
    let at_rev2 = if_(&e(0x93, &[&arg(1), &int(2)]), &ret(&buffer(mask.len() as u64, mask)));
    let query = is(0, &cat(&[&at_rev2, &ret(&buffer(1, &[0]))]));
    let body: Vec<u8> = [vec![query], rest.to_vec(), vec![ret(&int(0))]].concat().concat();
    method(&format!("{path}._DSM"), 4, &body)
}

pub fn machine() -> Vec<u8> {
    let ready = package(5, &[int(100_000), int(1_000), int(100_000), int(10_000), int(0)]);
    let host = dsm(
        "\\_SB.PCI0",
        &[0x61, 0x02],
        &[
            is(5, &ret(&int(0))),
            is(6, &cat(&[&store(&int(1), &name("\\F6RN")), &ret(&int(1))])),
            is(9, &ret(&ready)),
        ],
    );
    let label = |s: &[u8]| package(2, &[int(1), s.to_vec()]);
    cat(&[
        &name_obj("\\F6RN", &int(0)),
        &device("\\_SB.PCI0", &name_obj("_HID", &string("PNP0A08"))),
        &host,
        &device("\\_SB.PCI0.GFX0", &name_obj("_ADR", &int(0x0002_0000))),
        &dsm("\\_SB.PCI0.GFX0", &[0x81], &[is(7, &ret(&label(&string("Onboard IGD"))))]),
        &device("\\_SB.PCI0.NIC0", &name_obj("_ADR", &int(0x001F_0006))),
        &dsm("\\_SB.PCI0.NIC0", &[0x81], &[is(7, &ret(&label(&int(5))))]),
        &device("\\_SB.PCI0.NONE", &name_obj("_ADR", &int(0x0003_0000))),
    ])
}
