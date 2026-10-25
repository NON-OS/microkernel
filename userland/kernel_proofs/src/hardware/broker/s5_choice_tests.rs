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

//! F-111: which SLP_TYP pair soft-off writes. The model, stated apart: the
//! constant reader's pair when it has one within three bits, else the
//! interpreter's within three bits, else none; a pair wider than three
//! bits is refused, never cut. Cases: constant only, computed only, both
//! agreeing, both differing (a lie: the constant one stands), out of range
//! on either side, neither.

use super::super::acpi::s5_choice::{choose, posture, Source};

fn model(scan: Option<(u8, u8)>, interp: Option<(u8, u8)>) -> Option<((u8, u8), Source)> {
    let ok = |p: Option<(u8, u8)>| p.filter(|(a, b)| *a < 8 && *b < 8);
    ok(scan).map(|p| (p, Source::Scan)).or(ok(interp).map(|p| (p, Source::Interpreter)))
}

#[test]
fn the_table() {
    assert_eq!(choose(Some((7, 7)), None), Some(((7, 7), Source::Scan)), "constant");
    assert_eq!(choose(None, Some((5, 5))), Some(((5, 5), Source::Interpreter)), "computed");
    assert_eq!(choose(Some((7, 7)), Some((5, 0))), Some(((7, 7), Source::Scan)), "they differ");
    assert_eq!(choose(Some((8, 7)), Some((5, 5))), Some(((5, 5), Source::Interpreter)));
    assert_eq!(choose(None, Some((0x0F, 7))), None, "out of range: refused, never cut");
    assert_eq!(choose(None, None), None, "neither");
    assert_eq!(posture(None), b" soft_off=none");
    assert_eq!(posture(Some(Source::Scan)), b" soft_off=scan");
    assert_eq!(posture(Some(Source::Interpreter)), b" soft_off=interpreter");
}

#[test]
fn every_pair_is_its_model() {
    let values = [0u8, 1, 5, 7, 8, 0x0F, 0xFF];
    let pairs: Vec<Option<(u8, u8)>> = core::iter::once(None)
        .chain(values.iter().flat_map(|a| values.iter().map(move |b| Some((*a, *b)))))
        .collect();
    for scan in &pairs {
        for interp in &pairs {
            assert_eq!(choose(*scan, *interp), model(*scan, *interp), "{scan:?} {interp:?}");
        }
    }
}
