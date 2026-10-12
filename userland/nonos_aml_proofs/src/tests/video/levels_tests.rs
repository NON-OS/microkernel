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

//! _BCL decoded through the interpreter, and the steps the brightness keys
//! and a requested level take.

use crate::enc::*;
use crate::run::eval;
use crate::video::{change_of, nearest, parse_bcl, step, Change, Levels};

fn bcl(v: &[u64]) -> Option<Levels> {
    let e: Vec<Vec<u8>> = v.iter().map(|&x| int(x)).collect();
    parse_bcl(&eval(&ret(&package(e.len() as u8, &e)), vec![]).unwrap())
}

#[test]
fn levels_sorted_once_and_believed() {
    let l = bcl(&[80, 40, 100, 10, 40, 70, 101, 40]).unwrap();
    assert_eq!((l.on_ac, l.on_battery), (Some(80), Some(40)));
    assert_eq!(l.steps, [10, 40, 70, 100], "101 dropped, 40 once");
    assert_eq!(bcl(&[80, 40]), None, "no levels at all");
    assert_eq!(bcl(&[200, 300, 50]).map(|l| (l.on_ac, l.steps)), Some((None, vec![50])));
}

#[test]
fn keys_step_and_stop_at_the_ends() {
    let l = bcl(&[70, 40, 10, 40, 70, 100]).unwrap();
    assert_eq!(step(&l, Some(40), Change::Up), 70);
    assert_eq!(step(&l, Some(100), Change::Up), 100);
    assert_eq!(step(&l, Some(40), Change::Down), 10);
    assert_eq!(step(&l, Some(10), Change::Down), 10);
    assert_eq!(step(&l, Some(55), Change::Up), 70, "between levels");
    assert_eq!(step(&l, Some(55), Change::Down), 40);
    assert_eq!(step(&l, None, Change::Up), 100, "from the AC level, 70");
    assert_eq!(step(&l, Some(100), Change::Cycle), 10, "cycle wraps");
    assert_eq!(step(&l, Some(70), Change::Zero), 10);
}

#[test]
fn nearest_and_the_notify_values() {
    let l = bcl(&[70, 40, 10, 40, 70, 100]).unwrap();
    assert_eq!(
        (nearest(&l, 0), nearest(&l, 55), nearest(&l, 56), nearest(&l, 99)),
        (10, 40, 70, 100)
    );
    assert_eq!(nearest(&l, 25), 10, "a tie goes lower");
    let changes = [0x85, 0x86, 0x87, 0x88, 0x89, 0x80].map(change_of);
    assert_eq!(
        changes,
        [Some(Change::Cycle), Some(Change::Up), Some(Change::Down), Some(Change::Zero), None, None]
    );
}
