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

//! refundCommit: the wallet funds only a reply whose
//! refundCommit is keccak256(abi.encode(REFUND_TAG, refundTo, refundSalt))
//! over its own fresh refundTo and its own refundSalt.

use nonos_nox_config::REFUND_TAG;
use nonos_payment_proto::{OrderMade, OrderNew};
use sha3::{Digest, Keccak256};

use super::deposit_fixture::{made_at, ordered, salted, step_of, AT, K};
use super::fixture::{buy, offer, NOW, REFUND_PAIR, REFUND_SALT, REFUND_TO};
use crate::order::deposit_check::{deposit_check, REFUND_WRONG};
use crate::order::fees_kind::Fees;
use crate::order::fund_ask::fund_ask;
use crate::order::purchase::After;
use crate::order::read::Named;

fn commit(to: &[u8; 20], salt: &[u8; 32]) -> [u8; 32] {
    let mut w = [0u8; 32];
    w[12..].copy_from_slice(to);
    Keccak256::digest([&REFUND_TAG[..], &w, salt].concat()).into()
}

/// The meter's reply with refundCommit over `to` and `salt`, its id and deposit
/// consistent with it, and whether a Shield withdraw would be set up for it.
fn funds(n: &OrderNew, to: [u8; 20], salt: [u8; 32]) -> (bool, OrderMade) {
    let m = salted(n, 0xBB, Some(commit(&to, &salt)));
    let a = ordered(n, Ok(&m.encode()));
    let ok = deposit_check(&K, n, &m, Some(AT)).is_ok();
    let quoted = Fees::Quoted("0.02".into());
    let ask = fund_ask(&step_of(a.clone()), &Ok(Named::Escrow(buy())), &quoted).is_some();
    assert_eq!(ask, a == After::Fund(Box::new(m)), "funded exactly when Fund");
    assert_eq!(ok, ask);
    (ask, m)
}

#[test]
fn the_wallets_own_pair_funds_and_another_address_is_refused() {
    let n = buy().order_new(&offer(), REFUND_PAIR, NOW);
    assert_eq!(REFUND_TAG, <[u8; 32]>::from(Keccak256::digest(b"NONOS.nox.refund.v1")));
    assert_eq!(made_at(&n).terms[12], commit(&REFUND_TO, &REFUND_SALT));
    assert!(funds(&n, REFUND_TO, REFUND_SALT).0, "the right refundCommit funds");
    let (theirs, m) = funds(&n, [0x99; 20], REFUND_SALT);
    assert!(!theirs, "a refundCommit naming another address");
    assert_eq!(deposit_check(&K, &n, &m, Some(AT)), Err(REFUND_WRONG));
    for i in 0..32 {
        let mut salt = REFUND_SALT;
        salt[i] ^= 0x01;
        assert!(!funds(&n, REFUND_TO, salt).0, "refundSalt byte {i}");
    }
    for i in 0..20 {
        let mut to = REFUND_TO;
        to[i] ^= 0x80;
        assert!(!funds(&n, to, REFUND_SALT).0, "refundTo byte {i}");
    }
}
