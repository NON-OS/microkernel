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

//! settle's bondOf and isActive words, read strictly (B6).

use nonos_marketplace_abi::BondReading;

fn w(tail: &[u8]) -> [u8; 32] {
    let mut x = [0u8; 32];
    x[32 - tail.len()..].copy_from_slice(tail);
    x
}

#[test]
fn bond_words_read_strictly() {
    let words = [w(&[7; 20]), [8; 32], w(&500u128.to_be_bytes()), w(&0u64.to_be_bytes()), w(&[3])];
    let r = BondReading::from_words(&words, &[w(&[1])], 9).unwrap();
    assert_eq!(
        (r.payee, r.publisher_key, r.amount, r.unlock_at, r.slashed),
        ([7; 20], [8; 32], 500, 0, 3)
    );
    assert!(r.active && r.read_at == 9);
    assert!(!BondReading::from_words(&words, &[w(&[0])], 9).unwrap().active);
    assert_eq!(BondReading::from_words(&words, &[w(&[2])], 9), None, "a bool of 2");
    assert_eq!(BondReading::from_words(&words[..4], &[w(&[1])], 9), None, "a word short");
    for (i, at) in [(0, 11), (2, 15), (3, 23), (4, 0)] {
        let mut bad = words;
        bad[i][at] = 1;
        assert_eq!(BondReading::from_words(&bad, &[w(&[1])], 9), None, "word {i}");
    }
}
