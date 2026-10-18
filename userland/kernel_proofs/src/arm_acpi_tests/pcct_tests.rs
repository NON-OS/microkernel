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

//! PCCT: every subspace type read with its registers and timings, an
//! unknown type keeping its place (identifiers are positions), and
//! structures that do not fit refused.

use super::build::seal;
use super::pcct_build::*;
use crate::arm_acpi::pcct::pcct;
use crate::arm_acpi::sdt::TableError;

#[test]
fn every_type_in_its_place() {
    let unknown = [vec![9, 8], vec![0; 6]].concat();
    let subs =
        [generic(0, 0x1000), generic(1, 0x2000), generic(2, 0x3000), extended(3), extended(4)];
    let t = table(1, &[&subs[..], &[registers(), unknown]].concat());
    let p = pcct(&t).unwrap();
    assert!(p.interrupt_completion);
    assert_eq!(p.subspaces.len(), 7);
    let s0 = p.subspaces[0].unwrap();
    assert_eq!((s0.kind, s0.base, s0.length, s0.interrupt), (0, 0x1000, 0x1000, None));
    let bell = s0.doorbell.unwrap();
    assert_eq!((bell.gas.address, bell.preserve, bell.write), (0xFE10_0000, 0xFFFF_0000, 1));
    assert_eq!((s0.latency_us, s0.max_rate, s0.min_turnaround_us), (50, 1000, 10));
    assert_eq!(p.subspaces[1].unwrap().interrupt, Some((40, 1)));
    assert_eq!(p.subspaces[2].unwrap().ack.map(|a| a.gas.address), Some(0xFE10_0010));
    let s3 = p.subspaces[3].unwrap();
    assert_eq!((s3.base, s3.length, s3.interrupt), (0x8000_0000, 0x2000, Some((41, 3))));
    assert_eq!(s3.complete.map(|c| (c.gas.address, c.preserve)), Some((0xFE20_0010, 1)));
    assert_eq!(s3.error.map(|c| (c.gas.address, c.preserve)), Some((0xFE20_0018, 4)));
    assert_eq!((s3.latency_us, s3.max_rate, s3.min_turnaround_us), (60, 500, 20));
    assert_eq!(p.subspaces[4].unwrap().kind, 4);
    let s5 = p.subspaces[5].unwrap();
    assert_eq!(
        (s5.base, s5.length, s5.latency_us, s5.min_turnaround_us),
        (0x9000_0000, 0x400, 70, 30)
    );
    assert_eq!(s5.error.map(|e| e.preserve), Some(2));
    assert_eq!(p.subspaces[6], None, "an unknown type keeps its identifier");
}

#[test]
fn structures_that_do_not_fit() {
    let short = [vec![0, 20], vec![0; 18]].concat();
    assert_eq!(pcct(&table(0, &[short])).unwrap().subspaces, [None], "too short for type 0");
    let mut t = table(0, &[generic(0, 0x1000)]);
    t[48 + 1] = 70;
    seal(&mut t);
    assert_eq!(pcct(&t), Err(TableError::Structure(48)), "past the table");
    let mut t = table(0, &[generic(0, 0x1000)]);
    t[48 + 1] = 0;
    seal(&mut t);
    assert_eq!(pcct(&t), Err(TableError::Structure(48)), "length 0");
}
