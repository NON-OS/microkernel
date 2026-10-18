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

//! Finding the GPE methods under \_GPE and the wake events _PRW names.

use crate::consts::at;
use crate::enc::*;
use crate::events::{handlers, wake_gpes, Trigger};
use crate::exec::{ExecLimits, Machine};
use crate::load::Aml;
use crate::namespace::Id;
use crate::probe::Probe;
use crate::run::boot;

const NOP: &[u8] = &[0xA3];

#[test]
fn level_and_edge_methods_by_their_hex_numbers() {
    let src = cat(&[
        &method("\\_GPE._L01", 0, NOP),
        &method("\\_GPE._E1F", 0, NOP),
        &method("\\_GPE._LA0", 0, NOP),
        &method("\\_GPE._X02", 0, NOP),
        &method("\\_GPE._LXY", 0, NOP),
        &name_obj("\\_GPE._L03", &int(1)),
        &method("\\_GPE._L05", 0, NOP),
        &method("\\_GPE._E05", 0, NOP),
        &method("\\_SB._L06", 0, NOP),
    ]);
    let (aml, _) = boot(&src, 2);
    let (found, both) = handlers(&aml.ns);
    let got: Vec<_> = found.iter().map(|(n, h)| (*n, h.trigger)).collect();
    let want =
        [(1, Trigger::Level), (5, Trigger::Level), (0x1F, Trigger::Edge), (0xA0, Trigger::Level)];
    assert_eq!(got, want);
    assert_eq!(found[&1].method, at(&aml, "\\_GPE._L01").unwrap());
    assert_eq!(both, [5], "_L05 and _E05: the first defined is kept");
}

#[test]
fn wake_events_from_prw_and_a_bad_prw_reported() {
    let src = cat(&[
        &device("\\_SB.LID0", &name_obj("_PRW", &package(2, &[int(0x0D), int(3)]))),
        &device("\\_SB.PCI0", &device("XHC", &name_obj("_PRW", &package(2, &[int(0x6D), int(4)])))),
        &device(
            "\\_SB.GPED",
            &name_obj("_PRW", &package(2, &[package(2, &[int(1), int(2)]), int(3)])),
        ),
        &device("\\_SB.BAD", &name_obj("_PRW", &int(7))),
        &device("\\_SB.BIG", &name_obj("_PRW", &package(2, &[int(0x1_0000), int(3)]))),
    ]);
    let (mut aml, _) = boot(&src, 2);
    let mut probe = Probe::default();
    let mut eval = |aml: &mut Aml, id: Id| {
        Machine::new(aml, &mut probe, ExecLimits::DEFAULT).evaluate(aml, id, vec![])
    };
    let (wake, failed) = wake_gpes(&mut aml, &mut eval);
    assert_eq!(wake.into_iter().collect::<Vec<_>>(), [0x0D, 0x6D]);
    let bad: Vec<_> = failed.iter().map(|(id, _)| *id).collect();
    assert_eq!(bad, [at(&aml, "\\_SB.BAD._PRW").unwrap(), at(&aml, "\\_SB.BIG._PRW").unwrap()]);
}
