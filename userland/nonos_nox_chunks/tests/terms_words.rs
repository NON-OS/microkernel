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

//! The fourteen words read back into the terms exactly, and a word with bits
//! past its field's width is refused, so no reading has two encodings.

use nonos_nox_chunks::OrderTerms;

fn terms() -> OrderTerms {
    OrderTerms {
        listing_id: [1; 32],
        release_id: [2; 32],
        bond_id: [3; 32],
        payee: [4; 20],
        token: [5; 20],
        amount: 6_000_000_000_000_000_000,
        delivery_deadline: 7,
        dispute_window: 8,
        chunk_root: [9; 32],
        content_len: 10,
        cipher_root: [11; 32],
        key_commit: [12; 32],
        refund_commit: [13; 32],
        salt: [14; 32],
    }
}

#[test]
fn the_words_read_back_into_the_terms() {
    let w = terms().words();
    assert_eq!(OrderTerms::from_words(&w), Some(terms()));
    for (i, at) in [(3, 11), (4, 11), (5, 15), (6, 23), (7, 23), (9, 23)] {
        let mut bad = w;
        bad[i][at] = 1;
        assert_eq!(OrderTerms::from_words(&bad), None, "word {i} past its width");
    }
}
