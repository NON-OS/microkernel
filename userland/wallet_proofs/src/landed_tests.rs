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

//! Refunds and reclaims landed at fresh addresses (v1.4.52): the Shield's
//! handed list read newest first and bounded, and balanceOf's word read
//! only when it is one word that fits.

use crate::landed::{balance_of, line, parse_handed, MOST_READ};

fn word(n: u128) -> [u8; 32] {
    let mut w = [0; 32];
    w[16..].copy_from_slice(&n.to_be_bytes());
    w
}

#[test]
fn the_handed_list_reads_newest_first_and_skips_what_does_not_read() {
    let a = format!("0x{}", "ab".repeat(20));
    let values = [
        format!("3|{a}"),
        format!("9|{a}"),
        format!("9|{a}"),
        String::from("x|0x00"),
        format!("4|0x{}", "ab".repeat(19)),
        format!("5|{}", "ab".repeat(20)),
        format!("6{a}"),
        format!("4294967296|{a}"),
    ];
    let got = parse_handed(values.iter().map(|s| s.as_str()));
    assert_eq!(got.iter().map(|h| h.0).collect::<Vec<_>>(), [9, 3], "deduped, newest first");
    assert_eq!(got[0].1, [0xab; 20]);
    let many: Vec<String> = (0..40).map(|i| format!("{i}|{a}")).collect();
    let got = parse_handed(many.iter().map(|s| s.as_str()));
    assert_eq!((got.len(), got[0].0), (MOST_READ, 39), "the newest MOST_READ only");
}

#[test]
fn balance_of_is_one_word_that_fits() {
    assert_eq!(balance_of(&[word(7)]), Some(7));
    assert_eq!(balance_of(&[word(u128::MAX)]), Some(u128::MAX));
    let mut big = word(0);
    big[15] = 1;
    assert_eq!(balance_of(&[big]), None, "past u128");
    assert_eq!(balance_of(&[word(1), word(1)]), None, "two words");
    assert_eq!(balance_of(&[]), None, "none");
    assert_eq!(line(40, "12.5"), "fresh address 40: 12.5 NOX, a refund or a reclaim");
}
