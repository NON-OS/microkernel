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

//! Throttling through the interpreter (_TSS, _PTC, _TPC, _TSD), and the
//! rules: _TSS stops at a state malformed, past 100% or faster than the
//! one before; a _TSD whose count is not five is no domain.

use super::build::{ints, reg};
use crate::consts::at;
use crate::cpu::*;
use crate::enc::*;
use crate::object::Object;
use crate::probe::Probe;
use crate::run::{boot, eval};
use crate::tests::platform::run::evaluator;

fn value(v: &[u8]) -> Object {
    eval(&ret(v), vec![]).unwrap()
}
fn tss(states: &[[u64; 5]]) -> Vec<u8> {
    package(states.len() as u8, &states.iter().map(|s| ints(s)).collect::<Vec<_>>())
}

#[test]
fn throttling_tables_through_the_interpreter() {
    let states = [[100, 15000, 0, 0, 0], [75, 11000, 0, 0x1C, 0], [50, 7500, 0, 0x18, 0]];
    let ptc = package(2, &[reg(1, 4, 1, 0, 0x410), reg(1, 4, 1, 0, 0x410)]);
    let body = cat(&[
        &name_obj("_PTC", &ptc),
        &name_obj("_TSS", &tss(&states)),
        &method("_TPC", 0, &ret(&int(1))),
        &name_obj("_TSD", &package(1, &[ints(&[5, 0, 3, 0xFD, 4])])),
    ]);
    let (mut aml, _) = boot(&processor("\\_PR.CPU0", 1, 0x410, 6, &body), 2);
    let mut probe = Probe::default();
    let cpu0 = at(&aml, "\\_PR.CPU0").unwrap();
    let t = cpu_tables(&mut aml, &mut evaluator(&mut probe), cpu0);
    assert_eq!(t.tstates.iter().map(|s| s.percent).collect::<Vec<_>>(), [100, 75, 50]);
    assert_eq!((t.tstates[1].milliwatts, t.tstates[1].control), (11000, 0x1C));
    let ptc = t.ptc.unwrap();
    assert_eq!((ptc.control.space, ptc.control.address, ptc.control.offset), (1, 0x410, 1));
    assert_eq!(t.tpc, Some(1));
    assert_eq!(t.tsd, Some(Domain { domain: 3, coordination: 0xFD, processors: 4 }));
    assert!(t.pstates.is_empty() && t.pct.is_none(), "no _PSS here, only throttling");
}

#[test]
fn tss_stops_where_a_state_is_out_of_order_or_malformed() {
    let pct = |states: &[[u64; 5]]| -> Vec<u32> {
        parse_tss(&value(&tss(states))).iter().map(|s| s.percent).collect()
    };
    let s = |p: u64| [p, 1, 2, 3, 4];
    assert_eq!(pct(&[s(100), s(80), s(90), s(10)]), [100, 80], "faster after slower");
    assert_eq!(pct(&[s(101), s(50)]), Vec::<u32>::new(), "past 100%");
    assert_eq!(pct(&[s(100), s(0)]), [100], "0% is no state");
    assert_eq!(pct(&[s(100), s(50), s(50)]), [100, 50, 50], "equal steps stand");
    let short = package(2, &[ints(&s(100)), ints(&[50, 1, 2, 3])]);
    assert_eq!(parse_tss(&value(&short)).len(), 1, "four integers is no state");
    assert!(parse_tss(&value(&int(5))).is_empty());
    let tsd = |count| parse_tsd(&value(&package(1, &[ints(&[count, 0, 3, 0xFD, 4])])));
    assert_eq!((tsd(4), parse_tsd(&value(&int(5)))), (None, None));
}
