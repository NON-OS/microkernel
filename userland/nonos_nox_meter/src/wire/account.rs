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

//! The meter's state as payment_proto's ACCOUNT reply.

use nonos_payment_proto::{Account, AccountRow, ACCOUNT_ROWS_MAX, KEPT, READY};

use crate::state::MeterState;

/// `address` is the meter's, once its key is open; `kept` whether this
/// boot persists. `from` is the first channel of the page.
pub fn account(
    s: &MeterState,
    address: Option<[u8; 20]>,
    kept: bool,
    now: u64,
    from: usize,
) -> Account {
    let b = &s.budget;
    let rows = s.channels.iter().skip(from).take(ACCOUNT_ROWS_MAX).map(|c| AccountRow {
        channel_id: c.id,
        listing_id: c.listing_id,
        chain_id: c.chain_id,
        deposit: c.deposit,
        settled: c.settled,
        cumulative: c.cumulative,
        expiry: c.expiry,
        frozen: c.frozen,
    });
    Account {
        flags: if address.is_some() { READY } else { 0 } | if kept { KEPT } else { 0 },
        address: address.unwrap_or([0; 20]),
        cap: b.cap,
        spent: b.spent,
        expiry: b.expiry,
        left: b.left(now),
        total: s.channels.len() as u8,
        rows: rows.collect(),
    }
}
