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
//! true factors: no currency, whose rates it has no way to know. And the
//! decimals typed into it land where they were typed.

use crate::calc::actions::dispatch;
use crate::calc::buttons::Action;
use crate::calc::convert::{convert, list, CATEGORIES};
use crate::calc::fixed::FRAC;
use crate::calc::format::format;
use crate::calc::state::State;

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

fn typed(keys: &[Action]) -> (i128, String) {
    let mut state = State::new();
    keys.iter().for_each(|k| dispatch::run(&mut state, *k));
    let mut buf = [0u8; 64];
    let n = format(state.display, state.decimal_digits_typed, &mut buf);
    (state.display, String::from_utf8_lossy(&buf[..n]).into_owned())
}

#[test]
fn a_typed_decimal_is_the_number_typed() {
    let (one, point, five) = (Action::Digit(1), Action::Decimal, Action::Digit(5));
    assert_eq!(typed(&[one, point]), (FRAC, "1.".into()));
    assert_eq!(typed(&[one, point, five]), (FRAC * 3 / 2, "1.5".into()));
    assert_eq!(typed(&[point, Action::Digit(2), five]), (FRAC / 4, "0.25".into()));
    let eight = [Action::Digit(1); 8];
    let (all, shown) = typed(&[&[point][..], &eight, &[Action::Digit(9)]].concat());
    assert_eq!((all, shown.as_str()), (11_111_111, "0.11111111"), "a ninth is refused");
}
