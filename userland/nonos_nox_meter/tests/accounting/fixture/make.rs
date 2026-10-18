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

use nonos_nox_meter::budget::Budget;
use nonos_nox_meter::channel::{Channel, CHARGE_MARGIN, NO_SIG};
use nonos_nox_meter::state::MeterState;

pub const NOW: u64 = 1_800_000_000;
pub const PAYEE: [u8; 20] = [0xbb; 20];
pub const LISTING: [u8; 32] = [2; 32];
pub const ROOT: [u8; 32] = [5; 32];

/// A channel holding 1000 with nothing yet settled or signed.
pub fn channel() -> Channel {
    Channel {
        id: [9; 32],
        payee: PAYEE,
        listing_id: LISTING,
        chain_id: 1,
        open_nonce: 0,
        unit: [3; 32],
        deposit: 1000,
        settled: 0,
        cumulative: 0,
        seq: 0,
        expiry: NOW + 2 * CHARGE_MARGIN,
        usage_root: [0; 32],
        sig: NO_SIG,
        frozen: false,
        carried: 0,
        reclaim_to: [0x5e; 20],
    }
}

/// That channel, with a budget of 500 that lasts a day.
pub fn funded_state() -> MeterState {
    let budget = Budget { cap: 500, spent: 0, expiry: NOW + 86_400, listings: vec![] };
    let state = MeterState { budget, channels: vec![], last_settle: None, last_retry: 0 };
    state.with_channel(channel(), NOW).expect("an empty table has room")
}
