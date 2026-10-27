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

//! A listed-argument block's framing: its magic, its count, each length,
//! nothing after, and its whole size.

use super::args_tests::block;
use crate::args::{check, ArgsError as E, MAX_LEN, MAX_WORDS};

#[test]
fn the_framing_is_bounded_and_exact() {
    assert_eq!(check(b""), Err(E::Magic));
    assert_eq!(check(b"NLXARG01"), Err(E::Magic));
    assert_eq!(check(b"NLXARG02\x01\x03--a"), Err(E::Magic));
    assert_eq!(check(b"NLXARG01\x00"), Err(E::Count));
    assert_eq!(check(b"NLXARG01\x01"), Err(E::Short));
    assert_eq!(check(b"NLXARG01\x01\x00"), Err(E::Short));
    assert_eq!(check(b"NLXARG01\x01\x04--a"), Err(E::Short));
    assert_eq!(check(b"NLXARG01\x01\x03--a\x00"), Err(E::Trailing));
    assert_eq!(check(&block(&[b"--a", b"--a=1"])), Err(E::Twice));
    let many: Vec<Vec<u8>> = (0..=MAX_WORDS).map(|i| format!("--w{i}").into_bytes()).collect();
    let refs: Vec<&[u8]> = many.iter().map(|w| w.as_slice()).collect();
    assert_eq!(check(&block(&refs[..MAX_WORDS])), Ok(MAX_WORDS));
    assert_eq!(check(&block(&refs)), Err(E::Count));
    // Six words of 200 bytes pass MAX_LEN; five do not.
    let big: Vec<Vec<u8>> =
        (0..6u8).map(|i| [&[b'-', b'-', b'a' + i, b'='][..], &[b'v'; 196]].concat()).collect();
    let refs: Vec<&[u8]> = big.iter().map(|w| w.as_slice()).collect();
    assert!(block(&refs[..5]).len() <= MAX_LEN && check(&block(&refs[..5])) == Ok(5));
    assert_eq!(check(&block(&refs)), Err(E::Long));
}
