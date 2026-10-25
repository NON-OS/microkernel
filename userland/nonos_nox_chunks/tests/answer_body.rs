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

//! The offer inside a sealed answer has no count: n comes from the release's
//! content_len, and the body is exactly 64 + 1056 n bytes.

#[path = "common/answer_fixture.rs"]
mod answer_fixture;
#[path = "common/answer_len.rs"]
mod answer_len;
#[path = "common/k.rs"]
mod k;
#[path = "common/seller.rs"]
mod seller;

use answer_fixture::honest;
use answer_len::len;
use k::kec;
use nonos_nox_chunks::{offer_len, Offer};
use seller::KEY;

#[test]
fn the_body_is_exactly_its_length_for_the_release() {
    let o = honest();
    let b = o.encode();
    assert_eq!(b.len(), 64 + 1056 * 3);
    assert_eq!(offer_len(len()), Some(b.len()));
    assert_eq!(Offer::parse(&b, len()), Some(o.clone()));
    assert_eq!(Offer::parse(&b, 2048), None, "two chunks' worth is another length");
    assert_eq!(Offer::parse(&b[..b.len() - 1], len()), None, "short");
    assert_eq!(Offer::parse(&[b.as_slice(), &[0]].concat(), len()), None, "long");
    assert_eq!(offer_len(0), None, "no content, no offer");
    assert_eq!(offer_len(1), Some(64 + 1056));
    assert_eq!(o.key_commit, kec(&[&KEY]), "keyCommit is keccak256(abi.encode(K))");
}
