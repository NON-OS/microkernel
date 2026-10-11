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

use nonos_nox_meter::voucher::ChannelKey;
use nonos_payment_proto::OpenStart;
use nonos_settle_proto::{Gas, Opened};

use super::host::Host;

pub const NOX: [u8; 20] = [0x0a; 20];
pub const SETTLEMENT: [u8; 20] = [0x5e; 20];
pub const METER: [u8; 20] = [0xaa; 20];

pub fn start() -> OpenStart {
    OpenStart {
        chain_id: 11_155_111,
        payee: [0xbb; 20],
        listing_id: [2; 32],
        unit: [3; 32],
        amount: 1_000,
        expiry: 1_900_000_000,
        reclaim_to: [0xcc; 20],
    }
}

/// Enough ETH for both at a fee cap of 10.
pub fn gas() -> Gas {
    Gas { balance: 260_000 * 10, tx_nonce: 7, max_fee_per_gas: 10, max_priority_fee_per_gas: 2 }
}

/// The ChannelOpened this meter's open of start() emits, at openNonce 4.
pub fn event() -> Opened {
    let s = start();
    let key = ChannelKey {
        chain_id: s.chain_id,
        settlement: SETTLEMENT,
        payer: METER,
        payee: s.payee,
        listing_id: s.listing_id,
        open_nonce: 4,
    };
    Opened {
        channel_id: key.id(&Host),
        payer: METER,
        payee: s.payee,
        listing_id: s.listing_id,
        amount: s.amount,
        expiry: s.expiry,
        open_nonce: 4,
        reclaim_to: s.reclaim_to,
    }
}
