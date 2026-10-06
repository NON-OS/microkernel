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

//! The calculator's unit converter offers only conversions with fixed,
//! true factors: no currency, whose rates it has no way to know.

use crate::calc::convert::{convert, list, CATEGORIES};
use crate::calc::fixed::FRAC;

#[test]
fn the_converter_offers_no_currency() {
    let labels: Vec<&str> = CATEGORIES.iter().map(|c| c.label()).collect();
    assert_eq!(labels, vec!["Length", "Weight", "Temperature", "Data"]);
    for cat in CATEGORIES {
        for unit in list(cat) {
            assert!(!unit.name.contains("Dollar") && !unit.name.contains("Euro"), "{}", unit.name);
        }
    }
}

#[test]
fn the_remaining_conversions_still_hold() {
    let [length, _, temperature, _] = CATEGORIES;
    // 1 metre (index 2) is 100 centimetres (index 1).
    assert_eq!(convert(length, 2, 1, FRAC), Some(100 * FRAC));
    // 100 C is 212 F.
    assert_eq!(convert(temperature, 0, 1, 100 * FRAC), Some(212 * FRAC));
}
