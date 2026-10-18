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

//! A battery's state and charge, and an adapter's: _BST, the percentage,
//! _PSR, and packages refused.

use crate::enc::*;
use crate::power::{
    minutes_left, online, parse_bif, parse_bst, percent, BatteryInfo, BatteryStatus,
};
use crate::run::eval;

#[test]
fn bst_and_the_charge() {
    let bst = package(4, &[int(2), int(1200), int(2250), int(12100)]);
    let s = parse_bst(&eval(&ret(&bst), vec![]).unwrap()).unwrap();
    assert!(s.charging && !s.discharging && !s.critical);
    assert_eq!((s.rate, s.remaining, s.voltage), (Some(1200), Some(2250), Some(12100)));
    let info =
        BatteryInfo { last_full: Some(4500), design_capacity: Some(5000), ..Default::default() };
    assert_eq!(percent(&info, &s), Some(50));
    let over = BatteryStatus { remaining: Some(9000), ..s };
    assert_eq!(percent(&info, &over), Some(100), "never above full");
    let design_only =
        BatteryInfo { last_full: Some(0), design_capacity: Some(4500), ..Default::default() };
    assert_eq!(percent(&design_only, &s), Some(50));
    assert_eq!(percent(&BatteryInfo::default(), &s), None);
    assert_eq!(percent(&info, &BatteryStatus { remaining: None, ..s }), None);
}

#[test]
fn short_or_wrong_packages_are_refused_and_psr_reads() {
    assert_eq!(
        parse_bif(&eval(&ret(&package(3, &[int(1), int(2), int(3)])), vec![]).unwrap()),
        None
    );
    assert_eq!(parse_bst(&eval(&ret(&int(5)), vec![]).unwrap()), None);
    assert_eq!(online(&eval(&ret(&int(1)), vec![]).unwrap()), Some(true));
    assert_eq!(online(&eval(&ret(&int(0)), vec![]).unwrap()), Some(false));
}

#[test]
fn minutes_to_empty_and_to_full() {
    let info = BatteryInfo { last_full: Some(5000), ..Default::default() };
    let st = |discharging, charging, rate, remaining| BatteryStatus {
        discharging,
        charging,
        rate,
        remaining,
        ..Default::default()
    };
    assert_eq!(minutes_left(&info, &st(true, false, Some(1000), Some(2500))), Some(150));
    assert_eq!(minutes_left(&info, &st(false, true, Some(2000), Some(2500))), Some(75));
    assert_eq!(minutes_left(&info, &st(false, false, Some(1000), Some(2500))), None, "idle");
    assert_eq!(minutes_left(&info, &st(true, false, Some(0), Some(2500))), None);
    assert_eq!(minutes_left(&info, &st(true, false, None, Some(2500))), None);
}
