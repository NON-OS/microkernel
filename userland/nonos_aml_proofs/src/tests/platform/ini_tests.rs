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

//! The _INI walk: \_SB._INI first, then parents before children in
//! definition order; an absent device's subtree skipped, a functioning but
//! absent one's _INI skipped and its children's run.

use super::run::evaluator;
use crate::consts::at;
use crate::enc::*;
use crate::platform::initialize;
use crate::probe::Probe;
use crate::run::{boot, num};

/// An _INI that appends `k` to \ORD as a hex digit.
fn ini(k: u64) -> Vec<u8> {
    let shifted = e(0x77, &[&name("\\ORD"), &int(16), &[0]]);
    method("_INI", 0, &e(0x72, &[&shifted, &int(k), &name("\\ORD")]))
}

fn dev(path: &str, sta: Option<u64>, body: &[u8]) -> Vec<u8> {
    let sta = sta.map_or_else(Vec::new, |s| name_obj("_STA", &int(s)));
    device(path, &cat(&[&sta, body]))
}

#[test]
fn sb_first_then_depth_first_in_definition_order() {
    let src = cat(&[
        &name_obj("\\ORD", &int(0)),
        &dev("\\_SB.PCI0", None, &cat(&[&ini(2), &dev("LPC", None, &ini(3))])),
        &dev("\\_SB.GONE", Some(0), &cat(&[&ini(9), &dev("KID", None, &ini(9))])),
        &dev("\\_SB.DOCK", Some(8), &cat(&[&ini(9), &dev("BAY", None, &ini(4))])),
        &dev("\\_SB.LAST", Some(0xF), &ini(5)),
        &method("\\_SB._INI", 0, &e(0x70, &[&int(1), &name("\\ORD")])),
    ]);
    let (mut aml, _) = boot(&src, 2);
    let mut probe = Probe::default();
    let r = initialize(&mut aml, &mut evaluator(&mut probe));
    let ord = at(&aml, "\\ORD").unwrap();
    let got = num(&evaluator(&mut probe)(&mut aml, ord, vec![]).unwrap());
    assert_eq!(got, 0x12345, "\\_SB, PCI0, LPC, BAY, LAST");
    assert_eq!(r.absent, [at(&aml, "\\_SB.GONE").unwrap()]);
    assert_eq!(r.ran.len(), 5);
    assert!(r.ran.iter().all(|(_, x)| x.is_ok()));
}

#[test]
fn a_failing_sta_skips_its_device_and_is_reported() {
    let bad = method("_STA", 0, &e(0xA4, &[&e(0x83, &[&name("\\NONE")])]));
    let src = cat(&[&name_obj("\\ORD", &int(0)), &device("\\_SB.BAD", &cat(&[&bad, &ini(7)]))]);
    let (mut aml, _) = boot(&src, 2);
    let mut probe = Probe::default();
    let r = initialize(&mut aml, &mut evaluator(&mut probe));
    assert_eq!(r.sta_failed.len(), 1);
    assert!(r.ran.is_empty());
}
