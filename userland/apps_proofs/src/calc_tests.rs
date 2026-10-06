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

//! The calculator never wraps, panics or shows a wrong number on hostile
//! input: dividing by zero, overflowing and asking for the undefined each stop
//! the calculation, and the readout says which of the three it was.

use crate::calc::error_kind::ErrorKind;
use crate::calc::fixed::{Fixed, FRAC};
use crate::calc::format::{error_text, format, DISPLAY_MAX};
use crate::calc::op::{apply, Op};
use crate::calc::sci::{apply as sci, SciFn};
use crate::calc::unary::{reciprocal, sqrt, square};

fn n(v: i64) -> Fixed {
    v as Fixed * FRAC
}

#[test]
fn dividing_by_zero_stops_and_says_so() {
    assert_eq!(apply(n(7), 0, Op::Div), Err(ErrorKind::DivByZero));
    assert_eq!(reciprocal(0), Err(ErrorKind::DivByZero));
    assert_eq!(error_text(ErrorKind::DivByZero), "Cannot divide by zero");
}

#[test]
fn a_result_past_the_largest_value_stops_rather_than_wraps() {
    let big = n(9_999_999_999_999_999);
    assert_eq!(apply(big, big, Op::Mul), Err(ErrorKind::Overflow));
    assert_eq!(apply(Fixed::MAX, 1, Op::Add), Err(ErrorKind::Overflow));
    assert_eq!(apply(Fixed::MIN, 1, Op::Sub), Err(ErrorKind::Overflow));
    assert_eq!(square(big), Err(ErrorKind::Overflow));
    assert_eq!(apply(n(10), n(400), Op::Pow), Err(ErrorKind::Overflow));
    assert_eq!(error_text(ErrorKind::Overflow), "Result too large");
}

#[test]
fn asking_for_the_undefined_stops_and_says_so() {
    assert_eq!(sqrt(n(-4)), Err(ErrorKind::DomainError));
    assert_eq!(sci(SciFn::Ln, 0), Err(ErrorKind::DomainError));
    assert_eq!(sci(SciFn::Asin, n(2)), Err(ErrorKind::DomainError));
    assert_eq!(apply(n(-8), FRAC / 2, Op::Pow), Err(ErrorKind::DomainError));
    assert_eq!(error_text(ErrorKind::DomainError), "Not defined");
}

#[test]
fn each_stop_has_its_own_words() {
    let kinds = [ErrorKind::DivByZero, ErrorKind::Overflow, ErrorKind::DomainError];
    for (i, a) in kinds.iter().enumerate() {
        assert!(!error_text(*a).is_empty());
        for b in &kinds[i + 1..] {
            assert_ne!(error_text(*a), error_text(*b));
        }
    }
}

/// The readout's text for `value`, formatted into a display-sized buffer.
fn shown(value: Fixed) -> String {
    let mut buf = [0u8; DISPLAY_MAX];
    let len = format(value, 0, &mut buf);
    String::from_utf8(buf[..len].to_vec()).expect("ascii")
}

/// The value a readout text stands for, back in fixed point.
fn parse(text: &str) -> Fixed {
    let (neg, digits) = match text.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, text),
    };
    let (int, frac) = digits.split_once('.').unwrap_or((digits, ""));
    let mut frac = String::from(frac);
    while frac.len() < 8 {
        frac.push('0');
    }
    let whole: u128 = int.parse::<u128>().expect("digits") * FRAC as u128;
    let magnitude = whole + frac.parse::<u128>().expect("digits");
    if neg {
        (magnitude as Fixed).wrapping_neg()
    } else {
        magnitude as Fixed
    }
}

#[test]
fn the_widest_values_are_shown_whole() {
    for value in [Fixed::MIN, Fixed::MAX, Fixed::MIN + 1, -(n(1) / 3), n(-123_456_789)] {
        assert_eq!(parse(&shown(value)), value, "{value} shown as {}", shown(value));
    }
}

#[test]
fn the_most_negative_value_needs_more_than_the_old_width() {
    let text = shown(Fixed::MIN);
    assert_eq!(text, "-1701411834604692317316873037158.84105728");
    assert!(text.len() > 32);
}

#[test]
fn ordinary_arithmetic_is_untouched() {
    assert_eq!(apply(n(6), n(7), Op::Mul), Ok(n(42)));
    assert_eq!(apply(n(1), n(4), Op::Div), Ok(FRAC / 4));
    assert_eq!(sqrt(n(9)), Ok(n(3)));
}
