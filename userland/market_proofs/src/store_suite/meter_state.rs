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
//! The meter's state behind the store's channel rows: each row a channel
//! to the record's payee in its unit, under a budget that covers the
//! store's installs, so the store's cases turn on the row alone.

use nonos_payment_proto::AccountRow;

use super::held_fixture::NOW;
use crate::meter::budget::Budget;
use crate::meter::channel::{Channel, CHARGE_MARGIN, NO_SIG};
use crate::meter::state::MeterState;

/// The record's payee and unit (meter_fixture::record).
pub const PAYEE: [u8; 20] = [0; 20];
pub const UNIT: [u8; 32] = [3; 32];

/// The meter's channel that `r` is the ACCOUNT row of.
pub fn channel(r: &AccountRow) -> Channel {
    Channel {
        id: r.channel_id,
        payee: PAYEE,
        listing_id: r.listing_id,
        chain_id: r.chain_id,
        open_nonce: 1,
        unit: UNIT,
        deposit: r.deposit,
        settled: r.settled,
        cumulative: r.cumulative,
        seq: 1,
        expiry: r.expiry,
        usage_root: [0; 32],
        sig: NO_SIG,
        frozen: r.frozen,
        carried: 0,
        reclaim_to: r.reclaim_to,
    }
}

/// 100 to spend until past the store's channels, no listing capped.
pub fn budget() -> Budget {
    Budget { cap: 100, spent: 0, expiry: NOW + 4 * CHARGE_MARGIN, listings: Vec::new() }
}

pub fn state(channels: &[Channel]) -> MeterState {
    MeterState { budget: budget(), channels: channels.to_vec(), ..MeterState::default() }
}
