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

#[path = "common/k.rs"]
mod k;
#[path = "common/order_fixture.rs"]
mod order_fixture;

use nonos_nox_chunks::TermsWrong;
use order_fixture::check;

#[test]
fn the_agent_delivers_only_for_terms_it_rebuilt() {
    use TermsWrong::*;
    assert_eq!(check(|_| {}), Ok(()));
    assert_eq!(check(|t| t.amount = 5_000), Ok(()), "more than the price is fine");
    assert_eq!(check(|t| t.refund_commit = [0; 32]), Ok(()), "the buyer's own fields");
    assert_eq!(check(|t| t.listing_id[0] ^= 1), Err(NotMyRelease));
    assert_eq!(check(|t| t.release_id[0] ^= 1), Err(NotMyRelease));
    assert_eq!(check(|t| t.chunk_root[0] ^= 1), Err(NotMyRelease));
    assert_eq!(check(|t| t.content_len += 1), Err(NotMyRelease));
    assert_eq!(check(|t| t.cipher_root[0] ^= 1), Err(NotMyCipher));
    assert_eq!(check(|t| t.key_commit[0] ^= 1), Err(NotMyCipher));
    assert_eq!(check(|t| t.token[0] ^= 1), Err(NotMyPrice));
    assert_eq!(check(|t| t.payee[0] ^= 1), Err(NotMyPrice));
    assert_eq!(check(|t| t.bond_id[0] ^= 1), Err(NotMyPrice));
    assert_eq!(check(|t| t.amount = 999), Err(NotMyPrice));
}
