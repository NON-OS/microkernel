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

//! The packages' rules: _PSS stops at a bad state, _CST leaves bad ones
//! out, a short _CPC is refused, a register is a Generic Register.

use super::build::{cstate, ints, reg};
use crate::cpu::*;
use crate::enc::*;
use crate::object::Object;
use crate::run::eval;

fn value(v: &[u8]) -> Object {
    eval(&ret(v), vec![]).unwrap()
}

#[test]
fn pss_stops_where_a_state_is_malformed() {
    let pss = package(3, &[ints(&[2000, 1, 2, 3, 4, 5]), ints(&[1000, 1, 2, 3, 4]), ints(&[1; 6])]);
    let mhz: Vec<u32> = parse_pss(&value(&pss)).iter().map(|p| p.mhz).collect();
    assert_eq!(mhz, [2000], "_PPC counts by position, so nothing after the bad one");
    assert!(parse_pss(&value(&int(5))).is_empty());
    assert_eq!(
        [(0, 3), (2, 3), (5, 3), (u64::MAX, 3), (0, 0)].map(|(p, n)| ppc_index(p, n)),
        [0, 2, 2, 2, 0]
    );
}

#[test]
fn cst_believes_neither_count_nor_type() {
    let good = cstate(reg(1, 8, 0, 0, 0x414), 2, 50, 500);
    let cst = package(4, &[int(5), cstate(reg(1, 8, 0, 0, 0x415), 4, 1, 1), good.clone(), int(9)]);
    let kinds: Vec<u8> = parse_cst(&value(&cst)).iter().map(|c| c.kind).collect();
    assert_eq!(kinds, [2], "type 4 and a bare integer left out");
    assert!(parse_cst(&value(&package(2, &[int(0), good.clone()]))).is_empty(), "count 0");
    let many: Vec<Vec<u8>> = (0..12).map(|_| good.clone()).collect();
    let all = package(13, &[vec![int(255)], many].concat());
    assert_eq!(parse_cst(&value(&all)).len(), MAX_CSTATES);
}

#[test]
fn cpc_refused_when_short() {
    let short: Vec<u64> = [19, 3].into_iter().chain([0; 17]).collect();
    assert_eq!(parse_cpc(&value(&ints(&short))), None, "names fewer than 20");
    let lying: Vec<u64> = [23, 3].into_iter().chain([0; 19]).collect();
    assert_eq!(parse_cpc(&value(&ints(&lying))), None, "holds fewer than it names");
    let rev2: Vec<u64> = [21, 2].into_iter().chain(1..=19).collect();
    let cpc = parse_cpc(&value(&ints(&rev2))).unwrap();
    assert_eq!((cpc.revision, cpc.integer(HIGHEST), cpc.integer(LOWEST_MHZ)), (2, Some(1), None));
}

#[test]
fn registers_are_generic_register_descriptors() {
    let r = register_in(&[0x82, 0x0C, 0, 1, 8, 0, 1, 0x14, 4, 0, 0, 0, 0, 0, 0, 0x79, 0]).unwrap();
    assert_eq!((r.space, r.width, r.access, r.address), (SYSTEM_IO, 8, 1, 0x414));
    assert_eq!(register_in(&[0x82, 0x0C, 0, 1, 8, 0, 1, 0x14]), None, "cut short");
    assert_eq!(register_in(&[0x86, 0x09, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), None, "memory32");
    assert_eq!(register_in(&[0x82, 0x0B, 0, 1, 8, 0, 1, 0x14, 4, 0, 0, 0, 0, 0, 0]), None);
    assert!(register_in(&[0x82, 0x0C, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]).unwrap().is_null());
    let pct = package(2, &[reg(0x7F, 64, 0, 0, 0x199), int(0)]);
    assert_eq!(parse_pct(&value(&pct)), None, "status must be a register too");
}
