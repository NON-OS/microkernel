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

//! ORDER_STATUS over orderOf's five words (v1.4.26), and when the order
//! leaves the meter's table: Refunded, or Released with no extra, nothing
//! waiting, and the refund pair taken once.

use nonos_nox_meter::flow::{Flow, Next, Work};
use nonos_nox_meter::orders::Order;
use nonos_nox_meter::MeterError;
use nonos_payment_proto::OrderStatus;

use super::fixture::NOW;
use super::flow::{order, view};
use super::ret::{ret, word};

/// The answer after orderOf (state, funded, extra), a balance and a credit.
fn answer(o: Order, state: u128, extra: u128, waiting: (u128, u128)) -> Result<Next, MeterError> {
    let (mut f, _) = Flow::begin(Work::OrderStatus(o));
    let of = [ret(state), ret(900), ret(extra), ret(NOW.into()), ret(0)].concat();
    f.on_view(of).and_then(|_| f.on_view(ret(waiting.0))).and_then(|_| f.on_view(ret(waiting.1)))
}

fn body(state: u8, pending: u128) -> Vec<u8> {
    OrderStatus { state, funded: 900, pending, plain: false }.encode()
}

#[test]
fn a_status_reads_order_of_then_what_waits_at_the_deposit_address() {
    let o = order();
    let (mut f, first) = Flow::begin(Work::OrderStatus(o));
    assert_eq!(first, view(20, vec![o.order_id]));
    let of = [ret(2), ret(900), ret(0), ret(NOW.into()), ret(0)].concat();
    assert_eq!(f.on_view(of.clone()), Ok(view(1, vec![word(&o.deposit)])));
    assert_eq!(f.on_view(ret(70)), Ok(view(60, vec![ret(1)[0], word(&o.deposit)])));
    assert_eq!(f.on_view(ret(5)), Ok(Next::Done(body(2, 75))));
    assert_eq!(answer(o, 6, 0, (0, 0)), Err(MeterError::Mismatch), "no state 6");
    let (mut four, _) = Flow::begin(Work::OrderStatus(o));
    assert_eq!(
        four.on_view(of[..4].to_vec())
            .and_then(|_| four.on_view(ret(0)))
            .and_then(|_| four.on_view(ret(0))),
        Err(MeterError::Mismatch),
        "four words is the old orderOf"
    );
}

#[test]
fn an_order_leaves_the_table_only_when_nothing_is_left_to_refund() {
    let taken = Order { refund_taken: true, ..order() };
    let id = taken.order_id;
    assert_eq!(answer(taken, 5, 0, (0, 0)), Ok(Next::Drop(id, body(5, 0))), "refunded");
    assert_eq!(answer(taken, 4, 0, (0, 0)), Ok(Next::Drop(id, body(4, 0))), "released");
    assert_eq!(answer(taken, 4, 1, (0, 0)), Ok(Next::Done(body(4, 0))), "extra is the buyer's");
    assert_eq!(answer(taken, 5, 0, (1, 0)), Ok(Next::Done(body(5, 1))), "NOX still waits");
    assert_eq!(answer(taken, 5, 0, (0, 1)), Ok(Next::Done(body(5, 1))), "the pool credits it");
    assert_eq!(answer(taken, 3, 0, (0, 0)), Ok(Next::Done(body(3, 0))), "disputed");
    assert_eq!(answer(order(), 5, 0, (0, 0)), Ok(Next::Done(body(5, 0))), "the pair never taken");
}
