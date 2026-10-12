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

//! The download's fragments: 252 octets at most, the index counting 0 to
//! 7F and then from 1, the last marked with bit 7, and nothing lost.

use super::rtl::frag::{fragments, FRAG_LEN, LAST};

fn image(len: usize) -> Vec<u8> {
    (0..len).map(|i| (i * 7 + 3) as u8).collect()
}

#[test]
fn the_index_counts_to_7f_then_from_one() {
    let img = image(300 * FRAG_LEN + 17);
    let idx: Vec<u8> = fragments(&img).map(|(i, _)| i).collect();
    assert_eq!(idx.len(), 301);
    assert!(idx[..128].iter().enumerate().all(|(k, &i)| usize::from(i) == k));
    assert_eq!(idx[128..131], [1, 2, 3], "after 7F comes 1, not 0");
    assert_eq!(idx[254..256], [0x7f, 1]);
    assert_eq!(idx[300], (300 % 127) as u8 | LAST);
    assert!(idx[..300].iter().all(|i| i & LAST == 0), "only the last is marked");
}

#[test]
fn every_octet_goes_once_in_order() {
    for len in [1, 251, 252, 253, 504, 5000] {
        let img = image(len);
        let frags: Vec<(u8, &[u8])> = fragments(&img).collect();
        assert!(frags.iter().all(|(_, d)| d.len() <= FRAG_LEN));
        assert_eq!(frags.concat_data(), img, "{len}");
        assert_eq!(frags.len(), len / FRAG_LEN + 1);
    }
}

#[test]
fn an_exact_multiple_ends_with_an_empty_last_fragment() {
    let img = image(2 * FRAG_LEN);
    let frags: Vec<(u8, &[u8])> = fragments(&img).collect();
    assert_eq!(frags.len(), 3);
    assert_eq!(frags[2], (2 | LAST, &[][..]));
    let one: Vec<(u8, &[u8])> = fragments(&img[..10]).collect();
    assert_eq!(one, vec![(LAST, &img[..10])], "one short image is one fragment");
}

trait ConcatData {
    fn concat_data(&self) -> Vec<u8>;
}

impl ConcatData for Vec<(u8, &[u8])> {
    fn concat_data(&self) -> Vec<u8> {
        self.iter().flat_map(|(_, d)| d.iter().copied()).collect()
    }
}
