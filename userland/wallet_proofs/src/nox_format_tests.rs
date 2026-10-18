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

//! Known answers for the NOX figure and APR formatters and the hex digit.

use crate::hex_digit::hex_digit;
use crate::nox::format_apr::format_apr;
use crate::nox::format_nox::format_nox;

#[test]
fn format_nox_two_decimals() {
    let mut buf = [0u8; 48];
    let n = format_nox(1_500_000_000_000_000_000, &mut buf).expect("fits");
    assert_eq!(&buf[..n], b"1.50");
}

/* A buffer too short for the figure gives none, never a cut one. */
#[test]
fn format_nox_never_cuts_a_figure_short() {
    let mut small = [0u8; 4];
    assert_eq!(format_nox(12_345 * 1_000_000_000_000_000_000, &mut small), None);
    assert_eq!(format_nox(1_500_000_000_000_000_000, &mut small), Some(4));
    let mut max = [0u8; 48];
    assert!(format_nox(u128::MAX, &mut max).is_some());
}

#[test]
fn format_apr_percent() {
    let mut buf = [0u8; 16];
    let n = format_apr(1290, &mut buf);
    assert_eq!(&buf[..n], b"12.90%");
}

#[test]
fn hex_digit_alphabets() {
    assert_eq!(hex_digit(b'a' as u32), Some(10));
    assert_eq!(hex_digit(b'F' as u32), Some(15));
    assert_eq!(hex_digit(b'9' as u32), Some(9));
    assert_eq!(hex_digit(b'z' as u32), None);
}
