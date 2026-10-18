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

//! ACPI 4.0 fans: _FIF, _FST and _FPS decoded through the interpreter, and
//! the _FSL levels that follow.

use crate::enc::*;
use crate::fan::{levels, parse_fif, parse_fps, parse_fst, FanState};
use crate::run::eval;

const UNKNOWN: u64 = 0xFFFF_FFFF;

fn read<T>(v: Vec<u8>, f: impl Fn(&crate::object::Object) -> Option<T>) -> Option<T> {
    f(&eval(&ret(&v), vec![]).unwrap())
}

#[test]
fn fif_and_fst() {
    let i = read(package(4, &[int(0), int(1), int(10), int(1)]), parse_fif).unwrap();
    assert!(i.fine_grain && i.low_speed_notify);
    assert_eq!(i.step, 10);
    let s = read(package(3, &[int(0), int(50), int(2400)]), parse_fst).unwrap();
    assert_eq!((s.control, s.rpm), (Some(50), Some(2400)));
    let s = read(package(3, &[int(0), int(50), int(UNKNOWN)]), parse_fst).unwrap();
    assert_eq!(s.rpm, None);
    assert_eq!(read(package(2, &[int(0), int(1)]), parse_fst), None);
    assert_eq!(read(int(7), parse_fif), None);
}

#[test]
fn fps_states_and_the_levels() {
    let state = |c: u64, rpm: u64| package(5, &[int(c), int(UNKNOWN), int(rpm), int(0), int(900)]);
    let fps =
        package(4, &[int(0), state(10, 1200), package(2, &[int(1), int(2)]), state(90, 4000)]);
    let s = read(fps, parse_fps).unwrap();
    let want = |control, rpm| FanState { control, rpm: Some(rpm), power: Some(900) };
    assert_eq!(s, [want(10, 1200), want(90, 4000)], "the short state is skipped");
    assert_eq!(levels(false, &s), Some((90, 10)));
    assert_eq!(levels(true, &[]), Some((100, 0)));
    assert_eq!(levels(false, &[]), None);
    assert_eq!(read(package(0, &[]), parse_fps), None);
}
