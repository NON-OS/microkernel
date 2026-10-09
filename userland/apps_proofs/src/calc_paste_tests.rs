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

//! A paste into the calculator is a number or a sum entered as the keys that
//! type it; anything else is refused whole.

use crate::calc::op::Op;
use crate::calc::paste::{keys_for, PasteKey, PASTE_MAX};

/// The keys as the keypad labels them: digits, '.', the four operators,
/// 'n' for the sign key and '='.
fn pressed(text: &str, radix: Option<u32>) -> Option<String> {
    let keys = keys_for(text, radix)?;
    Some(
        keys.into_iter()
            .map(|k| match k {
                PasteKey::Digit(d) => char::from_digit(u32::from(d), 16).unwrap_or('?'),
                PasteKey::Point => '.',
                PasteKey::Op(Op::Add) => '+',
                PasteKey::Op(Op::Sub) => '-',
                PasteKey::Op(Op::Mul) => '*',
                PasteKey::Op(Op::Div) => '/',
                PasteKey::Op(_) => '?',
                PasteKey::Negate => 'n',
                PasteKey::Equals => '=',
            })
            .collect(),
    )
}

fn dec(text: &str) -> Option<String> {
    pressed(text, None)
}

#[test]
fn a_number_is_its_digits() {
    assert_eq!(dec("42").as_deref(), Some("42"));
    assert_eq!(dec("  3.14159\n").as_deref(), Some("3.14159"));
    assert_eq!(dec(".5").as_deref(), Some(".5"));
    assert_eq!(dec("+7").as_deref(), Some("7"));
}

#[test]
fn a_negative_number_presses_the_sign_after_its_digits() {
    assert_eq!(dec("-12.5").as_deref(), Some("12.5n"));
    assert_eq!(dec("\u{2212}3").as_deref(), Some("3n"), "the typographic minus");
}

#[test]
fn thousands_separators_group_three_digits_and_nothing_else() {
    assert_eq!(dec("1,234,567.89").as_deref(), Some("1234567.89"));
    assert_eq!(dec("1_000").as_deref(), Some("1000"));
    assert_eq!(dec("3,5"), None, "a decimal comma is ambiguous, so refused");
    assert_eq!(dec("1,2345"), None);
    assert_eq!(dec(",123"), None);
}

#[test]
fn a_sum_is_entered_left_to_right() {
    assert_eq!(dec("12+3*4").as_deref(), Some("12+3*4"));
    assert_eq!(dec("7 \u{d7} 6 =").as_deref(), Some("7*6="));
    assert_eq!(dec("100 \u{f7} 8 - 2").as_deref(), Some("100/8-2"));
    assert_eq!(dec("3 * -2 =").as_deref(), Some("3*2n="));
    assert_eq!(dec("5x5").as_deref(), Some("5*5"));
}

#[test]
fn garbage_is_refused_whole() {
    for junk in [
        "",
        "   ",
        "hello",
        "12abc",
        "2026-10-03 12:00",
        "1e5",
        "(1+2)",
        "1 234",
        "1+",
        "+",
        "=",
        "1==",
        "5 = 3",
        "\u{1b}[31m5",
        "12..5",
    ] {
        assert_eq!(dec(junk), None, "{junk:?}");
    }
}

#[test]
fn more_whole_digits_than_the_display_carries_is_refused() {
    assert_eq!(dec(&"9".repeat(16)).map(|s| s.len()), Some(16));
    assert_eq!(dec(&"9".repeat(17)), None, "the seventeenth digit would be dropped");
    assert!(dec(&"1".repeat(PASTE_MAX + 1)).is_none(), "too long to be a number");
}

#[test]
fn programmer_mode_takes_one_integer_in_its_base() {
    assert_eq!(pressed("ff", Some(16)).as_deref(), Some("ff"));
    assert_eq!(pressed("0xFF", Some(16)).as_deref(), Some("ff"));
    assert_eq!(pressed("0b1011", Some(2)).as_deref(), Some("1011"));
    assert_eq!(pressed("-17", Some(10)).as_deref(), Some("17n"));
    assert_eq!(pressed("0o17", Some(8)).as_deref(), Some("17"));
    assert_eq!(pressed("12", Some(2)), None, "2 is not a binary digit");
    assert_eq!(pressed("0x10", Some(10)), None, "a hex prefix in decimal mode");
    assert_eq!(pressed("1+1", Some(10)), None, "no sums in programmer mode");
}
