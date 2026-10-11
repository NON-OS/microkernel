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
use nonos_nox_meter::channel::{Channel, NO_SIG};
use nonos_nox_meter::state::MeterState;
use nonos_payment_proto::DrainSign;
use nonos_settle_proto::Gas;

pub const CHAIN: u64 = 11_155_111;
pub const NOW: u64 = 1_800_000_000;
pub const METER: [u8; 20] = [0xaa; 20];
pub const NOX: [u8; 20] = [0x0a; 20];
pub const SETTLEMENT: [u8; 20] = [0x5e; 20];

/// A channel holding 1000, `cumulative` of it signed with `sig`.
pub fn channel(id: u8, cumulative: u128, sig: [u8; 65]) -> Channel {
    Channel {
        id: [id; 32],
        payee: [0xbb; 20],
        listing_id: [2; 32],
        chain_id: CHAIN,
        open_nonce: id as u64,
        unit: [3; 32],
        deposit: 1000,
        settled: 0,
        cumulative,
        seq: 1,
        expiry: NOW + 30 * 86_400,
        usage_root: [5; 32],
        sig,
        frozen: false,
        carried: 0,
        reclaim_to: [0x5e; 20],
    }
}

/// Two channels with signed vouchers, 300 and 40, and one unsigned.
pub fn state() -> MeterState {
    let channels =
        vec![channel(1, 300, [0x11; 65]), channel(2, 40, [0x22; 65]), channel(3, 9, NO_SIG)];
    MeterState { budget: Budget::default(), channels, last_settle: None, last_retry: 0 }
}

pub fn sign(nonce: u64, ids: &[u8]) -> DrainSign {
    DrainSign {
        chain_id: CHAIN,
        nonce,
        gas_limit: 250_000,
        max_fee: 10,
        max_priority: 2,
        channels: ids.iter().map(|i| [*i; 32]).collect(),
    }
}

/// Enough ETH for a top-up at a fee cap of 10.
pub fn gas() -> Gas {
    Gas { balance: 180_000 * 10, tx_nonce: 7, max_fee_per_gas: 10, max_priority_fee_per_gas: 2 }
}
