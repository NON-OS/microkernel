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

/*
 * A transaction from refundTo (B8, kinds 10 and 11, v1.4.51): settle's
 * INTENT quote held to the config's Escrow, the kind's gas limit and the
 * meter's fee caps; the Shield ETH withdraw that pays its gas, the smallest
 * standard size at or above the most it can cost; SIGN_FROM_FRESH's request.
 */

use nonos_nox_config::{DISPUTE_GAS, REFUND_GAS};
use nonos_settle_proto::Quote;
use shield_wire::FreshAsk;

use super::send_bounds::held_to;
use super::send_step::{EscrowSend, DISPUTE};

const CENTI_ETH: u128 = 10_000_000_000_000_000;
/// The largest standard ETH size, 10 ETH, in hundredths.
const MOST_CENTS: u128 = 1_000;

/// Why settle's quote for `kind` is not signed, if it is not.
pub fn quote_refusal(q: &Quote, kind: u8, escrow: Option<[u8; 20]>) -> Option<&'static str> {
    let gas_max = if kind == DISPUTE { DISPUTE_GAS } else { REFUND_GAS };
    held_to(q, kind, escrow, gas_max)
}

/// The ETH withdraw for the gas: 1, 2 or 5 hundredths times a power of ten,
/// the smallest at or above gas times the fee, or None past 10 ETH.
pub fn gas_size(q: &Quote) -> Option<u128> {
    size_for((q.gas_limit as u128).checked_mul(q.max_fee)?)
}

/// The standard ETH withdraw at or above `cost` wei, or None past 10 ETH.
pub fn size_for(cost: u128) -> Option<u128> {
    let mut ten = 1u128;
    while ten <= MOST_CENTS {
        for m in [1u128, 2, 5] {
            let size = m * ten * CENTI_ETH;
            if m * ten <= MOST_CENTS && size >= cost {
                return Some(size);
            }
        }
        ten *= 10;
    }
    None
}

/// SIGN_FROM_FRESH's request for the send on `chain_id`, as quoted.
pub fn fresh_ask(d: &EscrowSend, q: &Quote, chain_id: u64) -> FreshAsk {
    FreshAsk {
        address: d.from,
        chain_id,
        nonce: q.nonce,
        gas: q.gas_limit,
        max_fee: q.max_fee,
        max_priority: q.max_priority,
        to: q.to,
        value: 0,
        data: d.calldata.clone(),
    }
}
