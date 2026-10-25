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

//! ORDER_REFUND (op 11, the plan): the wallet alone gets refundTo and
//! refundSalt of a kept order.

use alloc::vec::Vec;

use nonos_nox_meter::orders::Order;
use nonos_payment_proto::refund_reply;

use super::orders::order_id;
use crate::orders::{find, keep};
use crate::server::Call;

/// The refund pair, recorded as taken first: until then the order is
/// never dropped (v1.4.26).
pub fn order_refund(req: &Call<'_>) -> Vec<u8> {
    let o = order_id(req).and_then(|id| find(&id));
    let taken = o.and_then(|o| match o.refund_taken {
        true => Ok(o),
        false => keep(&Order { refund_taken: true, ..o }).map(|_| o),
    });
    req.result(taken.map(|o| refund_reply(&o.refund_to, &o.refund_salt)))
}
