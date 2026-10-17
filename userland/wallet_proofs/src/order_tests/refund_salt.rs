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

//! ORDER_NEW as the wallet sends it: its fresh refundTo and a refundSalt it
//! draws itself, only once the offer is in; no draw, no order.

use std::cell::Cell;

use nonos_payment_proto::EAGAIN;

use super::fixture::{buy, offer, NOW};
use crate::order::purchase::offer::NO_SALT;
use crate::order::purchase::{after_offer, After};

#[test]
fn the_request_carries_the_wallets_own_salt_and_fresh_refund_to() {
    let (b, fresh, drawn) = (buy(), [0x3C; 20], Cell::new(0));
    let draw = || {
        drawn.set(drawn.get() + 1);
        Some([0xD7; 32])
    };
    assert_eq!(after_offer(&b, Err(EAGAIN), (fresh, draw), NOW), After::Wait);
    assert_eq!(drawn.get(), 0, "nothing drawn before the offer is in");
    let After::Order(n) = after_offer(&b, Ok(&offer().encode()), (fresh, draw), NOW) else {
        panic!("no order")
    };
    assert_eq!((drawn.get(), n.refund_to, n.refund_salt), (1, fresh, [0xD7; 32]));
    assert_eq!(&n.encode()[252..], &[&fresh[..], &[0xD7; 32]].concat()[..], "the wire's tail");
    let none = after_offer(&b, Ok(&offer().encode()), (fresh, || None), NOW);
    assert_eq!(none, After::Failed(NO_SALT), "no salt, no order");
}
