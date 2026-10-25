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

/* Settle's BROADCAST as the wallet asks it, a dispute's, a refund's or a
 * publisher transaction's: chain id LE then the signed bytes; the answer
 * believed only as one 32-byte hash. */

use crate::wallet::settle::broadcast::{body, sent};

#[test]
fn the_body_is_the_chain_id_then_the_signed_bytes() {
    assert_eq!(
        body(11_155_111, &[0x02, 0xf8]),
        [&11_155_111u64.to_le_bytes()[..], &[2, 0xf8]].concat()
    );
    assert_eq!(body(1, &[]), 1u64.to_le_bytes().to_vec());
}

#[test]
fn only_one_whole_hash_is_sent() {
    assert_eq!(sent(Ok(vec![7; 32])), Some([7; 32]));
    for short_or_long in [vec![], vec![7; 31], vec![7; 33]] {
        assert_eq!(sent(Ok(short_or_long)), None);
    }
    assert_eq!(sent(Err(-110)), None);
}
