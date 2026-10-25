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

//! ORDER_DELIVERED with orderId alone (v1.4.46): the job asks settle's
//! DELIVERED for the order's key, then hands that key to the delivery.

use nonos_nox_config::Contract;
use nonos_nox_meter::flow::{Ask, Flow, Next, Work};

use super::flow::order;

#[test]
fn the_key_is_asked_of_settle_then_delivered() {
    let o = order();
    let work = Work::Deliver(o);
    assert_eq!((work.chain_id(), work.needs()), (o.chain_id, &[Contract::Escrow][..]));
    assert!(work.reads().is_empty(), "nothing is read before the key");
    let (mut flow, first) = Flow::begin(work);
    assert_eq!(first, Next::Ask(Ask::Delivered(o.order_id)));
    assert_eq!(flow.on_hash(&[9; 32]), Next::Deliver(o.order_id, [9; 32]));
}

#[test]
fn other_jobs_still_take_a_hash_as_a_broadcast() {
    let (mut flow, _) = Flow::begin(Work::OrderStatus(order()));
    assert_eq!(flow.on_hash(&[9; 32]), Next::Done(vec![9; 32]), "the hashes, not a key");
}
