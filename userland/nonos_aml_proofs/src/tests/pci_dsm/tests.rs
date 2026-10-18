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

//! The PCI `_DSM`: the revision firmware answers is found, only documented
//! functions it offers are read, and an answer of the wrong shape is none.

use super::rig::machine;
use crate::consts::at;
use crate::namespace::ROOT;
use crate::pci_dsm::*;
use crate::platform::{child, integer};
use crate::probe::Probe;
use crate::run::boot;
use crate::tests::platform::run::evaluator;

#[test]
fn host_facts_from_the_offered_documented_functions() {
    let (mut aml, _) = boot(&machine(), 2);
    let mut probe = Probe::default();
    let host = at(&aml, "\\_SB.PCI0").unwrap();
    assert_eq!(offered(&mut aml, &mut evaluator(&mut probe), host), Some((2, 0x261)));
    let f = host_facts(&mut aml, &mut evaluator(&mut probe), host);
    assert_eq!(f.preserve_boot_config, Some(true), "function 5 answered 0: keep it");
    let r = f.readiness.unwrap();
    assert_eq!(
        (r.reset_us, r.link_up_us, r.d3hot_to_d0_us, r.vf_enable_us),
        (100_000, 1_000, 10_000, 0)
    );
    let ran = child(&mut aml, &mut evaluator(&mut probe), ROOT, *b"F6RN", vec![]);
    assert_eq!(ran.and_then(|r| r.ok()).as_ref().and_then(integer), Some(0), "6 never asked");
}

#[test]
fn labels_of_the_right_shape_only() {
    let (mut aml, _) = boot(&machine(), 2);
    let mut probe = Probe::default();
    let mut label = |p: &str| {
        let id = at(&aml, p).unwrap();
        device_label(&mut aml, &mut evaluator(&mut probe), id)
    };
    let gfx = label("\\_SB.PCI0.GFX0").unwrap();
    assert_eq!((gfx.instance, gfx.text.as_slice()), (1, &b"Onboard IGD"[..]));
    assert_eq!(label("\\_SB.PCI0.NIC0"), None, "an integer where the string goes");
    assert_eq!(label("\\_SB.PCI0.NONE"), None, "no _DSM");
    assert_eq!(label("\\_SB.PCI0"), None, "7 not offered by the host bridge");
}
