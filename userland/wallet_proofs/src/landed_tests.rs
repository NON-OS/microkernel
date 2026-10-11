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

//! The Shield's batches for the restore scan (v1.4.56) read in index order,
//! each index once, at most BATCH_MAX; balanceOf's word only when one word
//! fits; reads spaced by a random 2 to 4 seconds.

use crate::shield::landed::{balance_of, line, parse_handed, spacing, BATCH_MAX, SPACE_MS};

fn word(n: u128) -> [u8; 32] {
    let mut w = [0; 32];
    w[16..].copy_from_slice(&n.to_be_bytes());
    w
}

#[test]
fn a_batch_reads_in_index_order_and_skips_what_does_not_read() {
    let a = format!("0x{}", "ab".repeat(20));
    let values = [
        format!("9|{a}"),
        format!("3|{a}"),
        format!("9|{a}"),
        String::from("x|0x00"),
        format!("4|0x{}", "ab".repeat(19)),
        format!("5|{}", "ab".repeat(20)),
        format!("6{a}"),
        format!("4294967296|{a}"),
    ];
    let got = parse_handed(values.iter().map(|s| s.as_str()));
    assert_eq!(got.iter().map(|h| h.0).collect::<Vec<_>>(), [3, 9], "ordered, each once");
    let many: Vec<String> = (0..500).map(|i| format!("{i}|{a}")).collect();
    let got = parse_handed(many.iter().map(|s| s.as_str()));
    assert_eq!((got.len(), got[0].0), (BATCH_MAX, 0), "at most one batch");
}

#[test]
fn balance_of_is_one_word_that_fits_and_reads_are_spaced() {
    assert_eq!(balance_of(&[word(u128::MAX)]), Some(u128::MAX));
    let mut big = word(0);
    big[15] = 1;
    assert_eq!(balance_of(&[big]), None, "past u128");
    assert_eq!(balance_of(&[word(1), word(1)]), None, "two words");
    assert_eq!((spacing(0), spacing(u16::MAX)), (SPACE_MS, 2 * SPACE_MS));
    assert!(line(&[0xab; 20], "12.5").starts_with("0xabababab…: 12.5 NOX"));
}
