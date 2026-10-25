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
 * The Bond's calls from the entry (plan/23), each only once why.rs takes
 * the entry for this account: the bond with the exact approve before it,
 * and the delivery agent as typed. Chain values come in from settle's
 * READ, so this is pure.
 */

use alloc::vec::Vec;

use nonos_marketplace_abi::{Keccak, MarketplaceEntry, SettleMode};
use nonos_nox_config::Contract;

use super::abi::uint;
use super::call::Publisher;
use super::why::{ours, Why};
use crate::wallet::event::address_text::{case_ok, pasted};
use crate::wallet::event::hex_digit::hex_digit;

pub(super) type Calls = Result<Vec<Publisher>, Why>;

/// Q1 and Q2: `min_bond` is READ 33, `allowance` READ 2's word. The
/// approve is skipped only when the allowance is exactly `amount`.
pub fn bond_of(
    k: &impl Keccak,
    e: &MarketplaceEntry,
    me: &[u8; 20],
    amount: u128,
    min_bond: u128,
    allowance: &[u8; 32],
) -> Calls {
    let c = ours(k, e, me)?;
    if amount == 0 || amount < min_bond {
        return Err(Why::BelowMinBond);
    }
    if c.settle_mode == SettleMode::Escrow && amount < c.unit_price {
        return Err(Why::BelowUnitPrice);
    }
    let bond = Publisher::Bond { publisher_key: e.publisher_pubkey, amount };
    let approve = Publisher::Approve { spender: Contract::PublisherBond, amount };
    Ok(if *allowance == uint(amount) { Vec::from([bond]) } else { Vec::from([approve, bond]) })
}

/// Q3: mixed case must be the EIP-55 spelling; one case carries none.
pub fn agent_of(k: &impl Keccak, e: &MarketplaceEntry, me: &[u8; 20], typed: &str) -> Calls {
    let c = ours(k, e, me)?;
    let hex = pasted(typed).map_err(|_| Why::NotAnAddress)?;
    if !case_ok(k, &hex) {
        return Err(Why::Checksum);
    }
    let digit = |c: u8| hex_digit(u32::from(c)).unwrap_or(0);
    let agent: [u8; 20] = core::array::from_fn(|i| digit(hex[2 * i]) << 4 | digit(hex[2 * i + 1]));
    if agent == [0; 20] {
        return Err(Why::ZeroAgent);
    }
    Ok(Vec::from([Publisher::SetDeliveryAgent { bond_id: c.bond_id, agent }]))
}
