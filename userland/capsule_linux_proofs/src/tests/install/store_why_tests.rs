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


//! Every store status vfs latches (capsule_vfs blk/status.rs) reads as a
//! reason on the `linux` command's line, never as a bare number.

use crate::store_why::store_why;

/// Every code `blk/status.rs` gives, with the one it never gives.
const CODES: [u32; 11] = [1, 2, 3, 6, 7, 8, 9, 10, 11, 12, 13];

#[test]
fn every_code_says_a_reason_in_words() {
    for code in CODES {
        let why = store_why(code);
        assert!(why.len() > 12, "{code}: {why}");
        assert!(!why.bytes().any(|b| b.is_ascii_digit()), "{code} reads as a number: {why}");
        assert!(why.is_ascii() && !why.contains('\n'), "{code}: one plain line");
    }
}

#[test]
fn no_disk_and_a_slow_disk_are_told_apart() {
    assert!(store_why(1).contains("no disk"));
    assert!(store_why(2).contains("did not answer in time"));
    assert_ne!(store_why(1), store_why(2));
    assert!(store_why(9).contains("damaged store entry"));
}
