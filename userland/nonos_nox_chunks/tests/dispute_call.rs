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

#[path = "common/terms.rs"]
mod terms;
#[path = "common/word.rs"]
mod word;

use nonos_nox_chunks::{dispute_call, Dispute};
use terms::terms;
use word::word;

#[test]
fn dispute_word_for_word() {
    let t = terms();
    let d = Dispute {
        index: 3,
        cipher_chunk: vec![0xCC; 1024],
        plain_hash: [0xDD; 32],
        cipher_proof: vec![[1; 32]; 2],
        leaf_proof: vec![[2; 32]],
    };
    let n = |v: u64| word(&v.to_be_bytes());
    let mut want = 0xd84d6ef9u32.to_be_bytes().to_vec();
    t.words().iter().for_each(|w| want.extend_from_slice(w));
    let (at_bytes, at_cipher) = (19 * 32, 19 * 32 + 32 + 1024);
    let at_leaf = at_cipher + 32 * 3;
    for w in [n(3), n(at_bytes), [0xDD; 32], n(at_cipher), n(at_leaf), n(1024)] {
        want.extend_from_slice(&w);
    }
    want.extend_from_slice(&[0xCC; 1024]);
    for w in [n(2), [1; 32], [1; 32], n(1), [2; 32]] {
        want.extend_from_slice(&w);
    }
    assert_eq!(dispute_call(&t, &d), Some(want));
    let past = Dispute { cipher_chunk: vec![0xCC; 1025], ..d };
    assert_eq!(dispute_call(&t, &past), None, "one past CHUNK is refused, not written");
}
