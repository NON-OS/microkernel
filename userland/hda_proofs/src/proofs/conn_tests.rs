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
//! Connection lists in both forms, with ranges.

use crate::controller::codec::conn::decode;
use crate::controller::codec::widget::MAX_CONN;

fn run(len: u32, words: &[u32]) -> Vec<u8> {
    let mut out = [0u8; MAX_CONN];
    let per = if len & 0x80 != 0 { 2 } else { 4 };
    let n = decode(len, |i| words.get(i as usize / per).copied(), &mut out);
    out[..n as usize].to_vec()
}

#[test]
fn a_short_list_is_four_entries_a_word_lowest_byte_first() {
    assert_eq!(run(5, &[0x0f0d_0c02, 0x0000_0021]), vec![0x02, 0x0c, 0x0d, 0x0f, 0x21]);
}

#[test]
fn a_long_list_is_two_sixteen_bit_entries_a_word() {
    assert_eq!(run(0x80 | 3, &[0x0003_0002, 0x0000_0010]), vec![0x02, 0x03, 0x10]);
}

#[test]
fn a_range_entry_connects_every_node_from_the_one_before_it() {
    // 0x02, then a range up to 0x05: 0x02 0x03 0x04 0x05.
    assert_eq!(run(2, &[0x0000_8502]), vec![0x02, 0x03, 0x04, 0x05]);
    assert_eq!(run(0x80 | 2, &[0x8006_0002]), vec![0x02, 0x03, 0x04, 0x05, 0x06]);
}

#[test]
fn a_list_longer_than_the_record_is_cut_without_shifting_indices() {
    let words: Vec<u32> = (0..10).map(|i| 0x0101_0101 * (i + 1)).collect();
    let got = run(40, &words);
    assert_eq!(got.len(), MAX_CONN);
    assert_eq!(got[4], 2, "entry 4 is the first of the second word");
}

#[test]
fn a_node_id_no_verb_can_address_is_kept_as_zero_in_its_place() {
    assert_eq!(run(0x80 | 2, &[0x0002_0200]), vec![0, 0x02]);
}
