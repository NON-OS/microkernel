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

//! HID over SPI's _DSM: the documented functions function 0 offers are
//! read, one it does not offer stays unknown, and an undocumented one is
//! never evaluated even when offered (`\F7RN` stays 0).

use crate::consts::at;
use crate::enc::*;
use crate::namespace::ROOT;
use crate::platform::{child, integer};
use crate::probe::Probe;
use crate::run::boot;
use crate::serial::{hid_spi, HidSpi};
use crate::tests::platform::run::evaluator;

fn machine() -> Vec<u8> {
    let f = |n: u64, body: &[u8]| if_(&e(0x93, &[&arg(2), &int(n)]), body);
    let body = cat(&[
        // Functions 0 to 5 and 7: not 6, the flags.
        &f(0, &ret(&buffer(1, &[0xBF]))),
        &f(1, &ret(&int(0x1000))),
        &f(2, &ret(&int(0x1004))),
        &f(3, &ret(&int(0x2000))),
        &f(4, &ret(&int(0x0B))),
        &f(5, &ret(&int(0x02))),
        &f(7, &cat(&[&store(&int(1), &name("\\F7RN")), &ret(&int(0xDEAD))])),
        &ret(&int(0)),
    ]);
    cat(&[
        &name_obj("\\F7RN", &int(0)),
        &device("\\_SB.SPI1", &name_obj("_HID", &string("TST0001"))),
        &device("\\_SB.SPI1.TPD0", &name_obj("_HID", &string("PNP0C51"))),
        &method("\\_SB.SPI1.TPD0._DSM", 4, &body),
        &device("\\_SB.SPI1.NODS", &name_obj("_HID", &string("PNP0C51"))),
    ])
}

#[test]
fn documented_and_offered_functions_only() {
    let (mut aml, _) = boot(&machine(), 2);
    let mut probe = Probe::default();
    let dev = at(&aml, "\\_SB.SPI1.TPD0").unwrap();
    let got = hid_spi(&mut aml, &mut evaluator(&mut probe), dev).unwrap();
    let want = HidSpi {
        input_header: Some(0x1000),
        input_body: Some(0x1004),
        output: Some(0x2000),
        read_opcode: Some(0x0B),
        write_opcode: Some(0x02),
        flags: None,
    };
    assert_eq!(got, want);
    let ran = child(&mut aml, &mut evaluator(&mut probe), ROOT, *b"F7RN", vec![]);
    assert_eq!(ran.and_then(|r| r.ok()).as_ref().and_then(integer), Some(0), "7 never asked");
    let none = at(&aml, "\\_SB.SPI1.NODS").unwrap();
    assert_eq!(hid_spi(&mut aml, &mut evaluator(&mut probe), none), None, "no _DSM");
}
