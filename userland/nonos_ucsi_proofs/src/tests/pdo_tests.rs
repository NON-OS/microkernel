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

//! The offers decoded: each kind of PDO from known figures, an augmented
//! kind not known left alone, and a request read against the offer it
//! names, refused for a position no offer holds.

use crate::contract::{contract, Contract};
use crate::pdo::{max_mw, pdo, Pdo};

pub fn fixed(mv: u32, ma: u32) -> u32 {
    ((mv / 50) << 10) | (ma / 10)
}

#[test]
fn every_kind_of_offer_decodes() {
    assert_eq!(pdo(fixed(20_000, 3_250)), Pdo::Fixed { mv: 20_000, ma: 3_250 });
    let battery = 1 << 30 | 420 << 20 | 100 << 10 | 240;
    assert_eq!(pdo(battery), Pdo::Battery { min_mv: 5_000, max_mv: 21_000, mw: 60_000 });
    let variable = 2 << 30 | 420 << 20 | 100 << 10 | 300;
    assert_eq!(pdo(variable), Pdo::Variable { min_mv: 5_000, max_mv: 21_000, ma: 3_000 });
    let pps = 3 << 30 | 210 << 17 | 33 << 8 | 60;
    assert_eq!(pdo(pps), Pdo::Pps { min_mv: 3_300, max_mv: 21_000, ma: 3_000 });
    assert_eq!(pdo(3 << 30 | 1 << 28), Pdo::Other, "an augmented kind not known");
    assert_eq!(max_mw(pdo(fixed(20_000, 3_250))), 65_000);
    assert_eq!(max_mw(pdo(battery)), 60_000);
    assert_eq!(max_mw(Pdo::Other), 0);
}

#[test]
fn a_request_reads_against_the_offer_it_names() {
    let offered = [fixed(5_000, 3_000), fixed(20_000, 3_250), 3 << 30 | 210 << 17 | 33 << 8 | 60];
    let rdo = |pos: u32, low: u32| pos << 28 | low;
    let c = contract(rdo(2, 300 << 10), &offered).unwrap();
    assert_eq!(c, Contract { position: 2, mv: 20_000, ma: 3_000, mw: 60_000 });
    let p = contract(rdo(3, 450 << 9 | 40), &offered).unwrap();
    assert_eq!(p, Contract { position: 3, mv: 9_000, ma: 2_000, mw: 18_000 }, "programmable");
    assert_eq!(contract(rdo(0, 0), &offered), None, "position 0 is no offer");
    assert_eq!(contract(rdo(4, 0), &offered), None, "past the offers");
    assert_eq!(contract(rdo(1, 0), &[]), None);
}
