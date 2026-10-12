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

//! Backlit outputs found by what they define: _BCL and _BCM both, under
//! the adapter or anywhere else.

use crate::consts::at;
use crate::enc::*;
use crate::run::boot;
use crate::run::eval;
use crate::video::{backlights, parse_dod};

#[test]
fn outputs_with_bcl_and_bcm() {
    let bcl = name_obj("_BCL", &package(3, &[int(100), int(50), int(100)]));
    let bcm = method("_BCM", 1, &[]);
    let lcd = device("LCD0", &cat(&[&name_obj("_ADR", &int(0x400)), &bcl, &bcm]));
    let crt = device("CRT0", &cat(&[&name_obj("_ADR", &int(0x100)), &bcl]));
    let gfx = device("GFX0", &cat(&[&method("_DOS", 1, &[]), &lcd, &crt]));
    let other = device("\\_SB.PNL1", &cat(&[&bcl, &method("_BCM", 1, &[])]));
    let (aml, _) = boot(&cat(&[&device("\\_SB.PCI0", &gfx), &other]), 2);
    let want = ["\\_SB.PCI0.GFX0.LCD0", "\\_SB.PNL1"].map(|p| at(&aml, p).unwrap());
    assert_eq!(backlights(&aml), want, "CRT0 has no _BCM");
}

#[test]
fn dod_lists_the_outputs() {
    let dod = package(3, &[int(0x8000_0100), int(0x8000_0400), string("x")]);
    assert_eq!(parse_dod(&eval(&ret(&dod), vec![]).unwrap()), [0x8000_0100, 0x8000_0400]);
    assert!(parse_dod(&eval(&ret(&int(1)), vec![]).unwrap()).is_empty());
}
