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

//! When an order leaves the meter's table: Refunded, or Released with no
//! extra, nothing waiting, and the refund pair taken once.

use nonos_nox_meter::flow::Next;
use nonos_nox_meter::orders::Order;

use super::flow::order;
use super::status::{answer, body};

#[test]
fn an_order_leaves_the_table_only_when_nothing_is_left_to_refund() {
    let taken = Order { refund_taken: true, ..order() };
    let id = taken.order_id;
    assert_eq!(answer(taken, 5, 0, (0, 0)), Ok(Next::Drop(id, body(5, 0, 0))), "refunded");
    assert_eq!(answer(taken, 4, 0, (0, 0)), Ok(Next::Drop(id, body(4, 0, 0))), "released");
    assert_eq!(
        answer(taken, 4, 1, (0, 0)),
        Ok(Next::Done(body(4, 0, 1))),
        "extra is the buyer's, and said"
    );
    assert_eq!(answer(taken, 5, 0, (1, 0)), Ok(Next::Done(body(5, 1, 0))), "NOX still waits");
    assert_eq!(answer(taken, 5, 0, (0, 1)), Ok(Next::Done(body(5, 1, 0))), "the pool credits it");
    assert_eq!(answer(taken, 3, 0, (0, 0)), Ok(Next::Done(body(3, 0, 0))), "disputed");
    assert_eq!(
        answer(order(), 5, 0, (0, 0)),
        Ok(Next::Done(body(5, 0, 0))),
        "the pair never taken"
    );
}
