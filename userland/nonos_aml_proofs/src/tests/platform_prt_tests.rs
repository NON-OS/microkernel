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

//! _PRT: entries straight to a GSI, and entries through a link device's
//! current resources, small IRQ and Extended Interrupt descriptors alike.

use super::platform_run::evaluator;
use crate::consts::at;
use crate::enc::*;
use crate::platform::{interrupts, routes, Route};
use crate::probe::Probe;
use crate::run::boot;

fn entry(slot: u64, pin: u64, source: &[u8], index: u64) -> Vec<u8> {
    package(4, &[int((slot << 16) | 0xFFFF), int(pin), source.to_vec(), int(index)])
}

#[test]
fn direct_and_linked_routes() {
    let irq11 = [0x22, 0x00, 0x08, 0x79, 0x00];
    let src = cat(&[
        &device("\\_SB.LNKA", &name_obj("_CRS", &buffer(5, &irq11))),
        &device(
            "\\_SB.PCI0",
            &name_obj(
                "_PRT",
                &package(2, &[entry(2, 0, &int(0), 16), entry(3, 1, &name("\\_SB.LNKA"), 0)]),
            ),
        ),
    ]);
    let (mut aml, _) = boot(&src, 2);
    let bridge = at(&aml, "\\_SB.PCI0").unwrap();
    let link = at(&aml, "\\_SB.LNKA").unwrap();
    let mut probe = Probe::default();
    let got = routes(&mut aml, &mut evaluator(&mut probe), bridge).unwrap().unwrap();
    assert_eq!(
        got,
        [
            Route { slot: 2, pin: 0, link: None, gsi: Some(16) },
            Route { slot: 3, pin: 1, link: Some(link), gsi: Some(11) },
        ]
    );
}

#[test]
fn resource_templates() {
    let ext = [0x89, 0x06, 0x00, 0x09, 0x01, 0x14, 0x00, 0x00, 0x00];
    let mixed =
        [&[0x47, 1, 0x60, 0, 0x60, 0, 1, 1][..], &[0x22, 0x02, 0x00], &ext, &[0x79, 0]].concat();
    assert_eq!(interrupts(&mixed), [1, 0x14], "an I/O descriptor stepped over");
    assert_eq!(interrupts(&[0x22, 0x00, 0x00, 0x79, 0]), [] as [u32; 0], "an empty mask");
    assert_eq!(interrupts(&ext[..7]), [] as [u32; 0], "cut short");
    assert_eq!(interrupts(&[0x79, 0x00, 0x22, 0x01, 0x00]), [] as [u32; 0], "after the end tag");
}
