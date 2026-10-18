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

//! Gas limits for the transactions the OS encodes itself (plan/21, v1.4.1).
//! Generous on purpose: the signer pays only for what is used, and a limit
//! too low reverts with the fee spent.

/// NOX approve(spender, amount).
pub const APPROVE_GAS: u64 = 60_000;
/// NoxSettlement open(payee, listingId, amount, expiry, reclaimTo).
pub const OPEN_GAS: u64 = 200_000;
/// NoxSettlement topUp(channelId, amount) (v1.4.7).
pub const TOPUP_GAS: u64 = 120_000;
/// NoxSettlement close(channelId) (v1.4.7).
pub const CLOSE_GAS: u64 = 120_000;
/// ShieldedPool claim(assetId, to): one token transfer out of the pool (v1.4.17).
pub const CLAIM_GAS: u64 = 150_000;
/// Escrow dispute(T, i, c_i, plainHash, proofs) from refundTo (v1.4.51):
/// two Merkle paths, a chunk hash and the bond's slash.
pub const DISPUTE_GAS: u64 = 400_000;
/// Escrow refund(T, refundTo, refundSalt) from refundTo (v1.4.51).
pub const REFUND_GAS: u64 = 150_000;
/// The most gas the meter signs one settle() for, whatever settle's estimate
/// says: twelve vouchers, each a signature recovery and a transfer.
pub const SETTLE_GAS_MAX: u64 = 3_000_000;

/// The most per gas the OS signs for: 1000 gwei, the wallet's FEE_CEILING_WEI
/// (plan/20, v1.4.3). The chain sets the base fee, so a lying fee
/// suggestion can overpay only the tip.
pub const MAX_FEE_PER_GAS: u128 = 1_000_000_000_000;
/// The most tip per gas the OS signs for: 10 gwei.
pub const MAX_PRIORITY_FEE_PER_GAS: u128 = 10_000_000_000;
