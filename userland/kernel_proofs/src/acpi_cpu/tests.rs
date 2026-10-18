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

use super::build::{header, laptop, reg, words};
use super::cpu_layout::*;
use super::cpu_wire::decode;

#[test]
fn a_whole_record_decodes() {
    let t = decode(&laptop()).expect("well formed");
    assert_eq!((t.uid, t.ppc), (4, Some(1)));
    let pct = t.pct.unwrap();
    assert_eq!((pct[0].space, pct[0].value, pct[1].value), (0x7F, 0x199, 0x198));
    let mhz: Vec<u32> = t.pstates.iter().map(|p| p.mhz).collect();
    assert_eq!(mhz, [2400, 800]);
    assert_eq!(t.pstates[0].control, 0x1800);
    let c: Vec<(u8, u8, u32)> =
        t.cstates.iter().map(|c| (c.kind, c.reg.space, c.latency_us)).collect();
    assert_eq!(c, [(1, 0x7F, 1), (2, 0x01, 50)]);
    let (rev, f) = t.cpc.unwrap();
    assert_eq!((rev, f[0].value, f[4].kind, f[5].space, f[20].value), (3, 100, ABSENT, 0x0A, 120));
}

#[test]
fn a_bare_processor_is_a_header() {
    let t = decode(&header(0, 0, 0, 9, 0)).unwrap();
    assert_eq!((t.uid, t.ppc, t.pct, t.cpc), (9, None, None, None));
    assert!(t.pstates.is_empty() && t.cstates.is_empty());
    assert_eq!(decode(&header(0, 0, 0, 9, 3)), None, "_PPC given but not flagged");
}

#[test]
fn length_counts_and_flags_must_agree() {
    let b = laptop();
    assert_eq!(decode(&b[..b.len() - 1]), None, "cut short");
    assert_eq!(decode(&[b.clone(), vec![0]].concat()), None, "trailing byte");
    assert_eq!(decode(&header(33, 0, 0, 0, 0)), None, "too many P-states");
    assert_eq!(decode(&header(0, 9, 0, 0, 0)), None, "too many C-states");
    assert_eq!(decode(&header(0, 0, 8, 0, 0)), None, "unknown flag");
    let mut v = b.clone();
    v[0] = 3;
    assert_eq!(decode(&v), None, "another version");
    let mut z = b;
    z[5] = 1;
    assert_eq!(decode(&z), None, "reserved byte set");
}

#[test]
fn registers_are_checked() {
    let one = |r: Vec<u8>| [header(0, 1, 0, 0, 0), r, words(&[1, 1, 1])].concat();
    assert!(decode(&one(reg(REGISTER, 0x01, 8, 0x414))).is_some());
    assert_eq!(decode(&one(reg(REGISTER, 0x02, 32, 0))), None, "PCI config space");
    assert_eq!(decode(&one(reg(INTEGER, 0, 0, 5))), None, "_CST needs a register");
    assert_eq!(decode(&one(reg(ABSENT, 0, 0, 0))), None);
    assert_eq!(decode(&one(reg(3, 0, 0, 0))), None, "unknown kind");
    let kind =
        |k: u32| [header(0, 1, 0, 0, 0), reg(REGISTER, 1, 8, 0x414), words(&[k, 1, 1])].concat();
    assert_eq!((decode(&kind(0)), decode(&kind(4))), (None, None), "types 1 to 3 only");
    let mut access = one(reg(REGISTER, 0x01, 8, 0x414));
    access[HEADER + 4] = 5;
    assert_eq!(decode(&access), None, "access size past QWord");
}
