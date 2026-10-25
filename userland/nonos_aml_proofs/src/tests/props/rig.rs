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

//! A camera sensor as ARM ACPI firmware describes it: properties of every
//! kind, and subnodes linked by path and by reference, an endpoint linking
//! back to its port (a loop), and a link to nothing.

use crate::enc::*;
use crate::props::{DEVICE_PROPERTIES_UUID, HIERARCHICAL_DATA_UUID};

fn pair(name: &str, v: Vec<u8>) -> Vec<u8> {
    package(2, &[string(name), v])
}

/// A _DSD-shaped package: the property set, then the links if any.
pub fn dsd(props: &[Vec<u8>], links: &[Vec<u8>]) -> Vec<u8> {
    let mut sets = vec![buffer(16, &DEVICE_PROPERTIES_UUID), package(props.len() as u8, props)];
    if !links.is_empty() {
        sets.push(buffer(16, &HIERARCHICAL_DATA_UUID));
        sets.push(package(links.len() as u8, links));
    }
    package(sets.len() as u8, &sets)
}

pub fn camera() -> Vec<u8> {
    let props = [
        pair("rotation", int(180)),
        pair("compatible", string("sony,imx219")),
        pair("clock", name("\\_SB.CLK0")),
        pair(
            "reset-gpios",
            package(1, &[package(4, &[name("\\_SB.GPO0"), int(3), int(0), int(1)])]),
        ),
        pair("too-deep", (0..6).fold(int(7), |v, _| package(1, &[v]))),
        pair("blob", buffer(2, &[1, 2])),
    ];
    let links = [
        pair("port@0", string("PRT0")),
        pair("port@1", name("\\_SB.CAM0.PRT1")),
        pair("port@9", string("NONE")),
    ];
    let prt0 = dsd(&[pair("reg", int(0))], &[pair("endpoint@0", string("EP00"))]);
    let ep00 =
        dsd(&[pair("data-lanes", package(2, &[int(1), int(2)]))], &[pair("back", string("PRT0"))]);
    let prt1 = dsd(&[pair("reg", int(1))], &[]);
    let body = cat(&[
        &name_obj("_HID", &string("SONY0219")),
        &name_obj("_DSD", &dsd(&props, &links)),
        &name_obj("PRT0", &prt0),
        &name_obj("EP00", &ep00),
        &name_obj("PRT1", &prt1),
    ]);
    cat(&[
        &device("\\_SB.CLK0", &[]),
        &device("\\_SB.GPO0", &[]),
        &device("\\_SB.CAM0", &body),
        &device("\\_SB.BARE", &name_obj("_HID", &string("ABCD0001"))),
    ])
}
