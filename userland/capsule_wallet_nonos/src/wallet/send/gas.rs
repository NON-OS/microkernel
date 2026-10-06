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

//! What the node's readings allow a payment to be signed with: the gas
//! limit from its estimate, and whether its fee may be signed on at all.
//! Pure, so wallet_proofs holds the same numbers.

/// A plain value transfer to an account costs exactly this.
pub const PLAIN_GAS: u64 = 21_000;

/// Headroom over a contract call's estimate, in percent: the state the node
/// estimated against can move before the transaction lands.
pub const MARGIN: u64 = 20;

/*
 * No transfer is signed above this gas price. Mainnet runs at a few gwei
 * and its worst spikes have been hundreds; a reading past a thousand is a
 * broken or lying node, and the fee cap is twice the reading, so signing on
 * it would spend real money on nothing.
 */
pub const FEE_CEILING_WEI: u64 = 1_000_000_000_000;

/// The gas limit for the node's `estimate`. A plain transfer (no call data)
/// is signed at exactly its fixed cost; anything else carries the margin.
pub fn gas_limit(estimate: u64, plain: bool) -> u64 {
    if plain && estimate <= PLAIN_GAS {
        return PLAIN_GAS;
    }
    estimate.saturating_add(estimate.saturating_mul(MARGIN) / 100)
}

/// Why a payment must not be signed on this fee reading, or None when it
/// may. No reading at all, or a zero, is a network that was not reached.
pub fn fee_refusal(fee_wei: Option<u64>) -> Option<&'static [u8]> {
    match fee_wei {
        None => Some(b"the network fee did not come with this reading, try again"),
        Some(0) => {
            Some(b"the node said the network fee is zero, which cannot be right, not signing")
        }
        Some(f) if f > FEE_CEILING_WEI => Some(b"network fee above 1000 gwei, not signing"),
        Some(_) => None,
    }
}
