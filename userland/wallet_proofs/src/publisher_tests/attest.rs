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

use nonos_marketplace_abi::{attest_words as market_words, SettleMode};

use super::fixture::{entry, K, KEY, ME};
use crate::attest::{decide, Decision};
use crate::publisher::{attest_of, attest_rows, Refused};
use crate::typed::attest_words;

#[test]
fn an_escrow_attest_is_the_markets_own_words() {
    let e = entry(SettleMode::Escrow);
    let a = attest_of(&K, &e, 1, 11155111).unwrap();
    assert_eq!(
        (a.chain_id, a.chunk_root, a.content_len, a.package_hash),
        (11155111, [0xcc; 32], 4096, [0; 32])
    );
    let mine = attest_words(&K, &ME, &KEY, &a).unwrap();
    let c = e.commerce.as_ref().unwrap();
    assert_eq!(
        mine,
        market_words(&K, &e, c, &e.releases[1]),
        "what is signed is what the market checks"
    );
    assert_eq!(decide(&[], &a), Ok(Decision::Sign));
    let rows = attest_rows(&e, 1, &a);
    assert_eq!((rows[1].1.as_str(), rows[3].1.as_str()), ("qwen-small@2", "4096 bytes"));
}

#[test]
fn a_package_attest_and_the_refusals() {
    let e = entry(SettleMode::Voucher);
    let a = attest_of(&K, &e, 0, 1).unwrap();
    assert_eq!((a.chunk_root, a.content_len, a.package_hash), ([0; 32], 0, [0x9a; 32]));
    assert!(attest_words(&K, &ME, &KEY, &a).is_ok());
    assert_eq!(attest_of(&K, &e, 2, 1).err(), Some(Refused::NoRelease));
    let mut unsold = e.clone();
    unsold.commerce = None;
    assert_eq!(attest_of(&K, &unsold, 0, 1).err(), Some(Refused::NotSold));
}
