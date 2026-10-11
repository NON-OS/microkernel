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

//! Kani harnesses: the dispute window's end and the gas withdraw's size
//! never panic, and a size, when there is one, covers the gas's cost.

use crate::order::order_of::window_end;
use crate::order::send_quote::gas_size;

#[kani::proof]
fn window_end_and_gas_size_never_panic() {
    let terms: [[u8; 32]; 14] = kani::any();
    let m = nonos_payment_proto::OrderMade { order_id: [0; 32], deposit: [0; 20], terms };
    let at: u64 = kani::any();
    let _ = window_end(&m, at);
    let (gas_limit, max_fee): (u64, u128) = (kani::any(), kani::any());
    let q = nonos_settle_proto::Quote {
        kind: 10,
        to: [0; 20],
        nonce: 0,
        gas_limit,
        max_fee,
        max_priority: 0,
    };
    if let Some(size) = gas_size(&q) {
        assert!(size >= q.gas_limit as u128 * q.max_fee);
    }
}
