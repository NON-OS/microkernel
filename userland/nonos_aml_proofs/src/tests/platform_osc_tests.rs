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

//! _OSC: queried, then committed with no more than was granted.

use super::platform_run::evaluator;
use crate::consts::at;
use crate::enc::*;
use crate::platform::{negotiate, OscError, PCI_HOST_UUID, PLATFORM_UUID};
use crate::probe::Probe;
use crate::run::{boot, num};

/// Grants only support bit 4 and control bit 0, counts its calls in \CNT,
/// and keeps the last status word it was passed in \LST.
fn osc_for(uuid_byte0: u8) -> Vec<u8> {
    let field = |i: u64, n: &str| e(0x8A, &[&arg(3), &int(i), &name(n)]);
    let unknown = e(0x7D, &[&name("STS0"), &int(4), &name("STS0")]);
    let body = cat(&[
        &field(0, "STS0"),
        &field(4, "SUP0"),
        &field(8, "CTL0"),
        &e(0x75, &[&name("\\CNT")]),
        &store(&name("STS0"), &name("\\LST")),
        &e(0x7B, &[&name("SUP0"), &int(0x10), &name("SUP0")]),
        &e(0x7B, &[&name("CTL0"), &int(0x01), &name("CTL0")]),
        &if_(
            &e(0x93, &[&e(0x83, &[&e(0x88, &[&arg(0), &int(0), &[0]])]), &int(uuid_byte0 as u64)]),
            &[],
        ),
        &else_(&unknown),
        &ret(&arg(3)),
    ]);
    cat(&[
        &name_obj("\\CNT", &int(0)),
        &name_obj("\\LST", &int(0)),
        &device("\\_SB.PCI0", &method("_OSC", 4, &body)),
    ])
}

#[test]
fn query_then_commit_what_was_granted() {
    let (mut aml, _) = boot(&osc_for(PCI_HOST_UUID[0]), 2);
    let osc = at(&aml, "\\_SB.PCI0._OSC").unwrap();
    let mut probe = Probe::default();
    let mut eval = evaluator(&mut probe);
    let got = negotiate(&mut aml, &mut eval, osc, (&PCI_HOST_UUID, 1), &[0, 0x11, 0x1F]).unwrap();
    assert_eq!(got[1..], [0x10, 0x01]);
    // Two calls, the query then the commit, which cleared the query flag.
    for (path, want) in [("\\CNT", 2), ("\\LST", 0)] {
        let id = at(&aml, path).unwrap();
        assert_eq!(num(&eval(&mut aml, id, vec![]).unwrap()), want, "{path}");
    }
}

#[test]
fn an_unknown_uuid_is_refused() {
    let (mut aml, _) = boot(&osc_for(PCI_HOST_UUID[0]), 2);
    let osc = at(&aml, "\\_SB.PCI0._OSC").unwrap();
    let mut probe = Probe::default();
    let r = negotiate(&mut aml, &mut evaluator(&mut probe), osc, (&PLATFORM_UUID, 1), &[0, 0, 0]);
    assert_eq!(r, Err(OscError::UnrecognizedUuid));
}
