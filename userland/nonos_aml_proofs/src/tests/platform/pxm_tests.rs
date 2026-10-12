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

//! Proximity domains: a device's own _PXM, a method's answer, a root
//! bridge's domain covering the devices below it, the nearest declaring
//! ancestor winning over a farther one, and none where no one declares.

use crate::consts::at;
use crate::enc::*;
use crate::platform::pxm;
use crate::probe::Probe;
use crate::run::boot;
use crate::tests::platform::run::evaluator;

fn machine() -> Vec<u8> {
    let pci1 = cat(&[
        &name_obj("_PXM", &int(1)),
        &device("RP01", &device("NVME", &[])),
        &device("RP02", &name_obj("_PXM", &int(3))),
        &device("GPU0", &method("_PXM", 0, &ret(&int(2)))),
    ]);
    cat(&[
        &device("\\_SB.PCI1", &pci1),
        &device("\\_SB.PCI0", &device("XHC0", &[])),
        &device("\\_SB.BAD0", &name_obj("_PXM", &string("one"))),
    ])
}

#[test]
fn own_inherited_and_none() {
    let (mut aml, _) = boot(&machine(), 2);
    let mut probe = Probe::default();
    let mut of = |aml: &mut _, path: &str| {
        let id = at(aml, path).unwrap();
        pxm(aml, &mut evaluator(&mut probe), id)
    };
    assert_eq!(of(&mut aml, "\\_SB.PCI1"), Some(1));
    assert_eq!(of(&mut aml, "\\_SB.PCI1.RP01.NVME"), Some(1), "the root bridge's domain");
    assert_eq!(of(&mut aml, "\\_SB.PCI1.RP02"), Some(3), "its own, nearer");
    assert_eq!(of(&mut aml, "\\_SB.PCI1.GPU0"), Some(2), "a method's answer");
    assert_eq!(of(&mut aml, "\\_SB.PCI0.XHC0"), None);
    assert_eq!(of(&mut aml, "\\_SB.BAD0"), None, "a _PXM that is no integer");
}
