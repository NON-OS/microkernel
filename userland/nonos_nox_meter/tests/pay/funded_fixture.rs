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

//! A record and a meter state for FUNDED: the fixture listing at 7 a unit
//! on chain 1, its channel with room for two installs, a budget for more.

use nonos_marketplace_abi::Commerce;
use nonos_nox_meter::budget::Budget;
use nonos_nox_meter::channel::{Channel, CHARGE_MARGIN, NO_SIG};
use nonos_nox_meter::pay::Offer;
use nonos_nox_meter::state::MeterState;

use super::fixture::{commerce, offer};

pub const NOW: u64 = 1_800_000_000;

/// One unit is a charge PAY takes: its minimum is the unit price.
pub fn sold() -> Commerce {
    Commerce { min_charge: 7, ..commerce() }
}

pub fn record() -> Offer {
    offer(&sold())
}

/// Listing [1; 32] on chain 1, to the record's payee in its unit: deposit
/// 20, max(cumulative, settled) 6.
pub fn channel() -> Channel {
    Channel {
        id: [5; 32],
        payee: [0xbb; 20],
        listing_id: [1; 32],
        chain_id: 1,
        open_nonce: 1,
        unit: [3; 32],
        deposit: 20,
        settled: 2,
        cumulative: 6,
        seq: 1,
        expiry: NOW + 2 * CHARGE_MARGIN,
        usage_root: [0; 32],
        sig: NO_SIG,
        frozen: false,
        carried: 0,
        reclaim_to: [0; 20],
    }
}

/// 100 to spend until past the channel, no listing capped.
pub fn budget() -> Budget {
    Budget { cap: 100, spent: 0, expiry: NOW + 4 * CHARGE_MARGIN, listings: Vec::new() }
}

pub fn state(channels: &[Channel]) -> MeterState {
    MeterState { budget: budget(), channels: channels.to_vec(), ..MeterState::default() }
}
