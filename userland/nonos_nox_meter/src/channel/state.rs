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
//! One prefunded channel (plan/21): the meter's view of what was deposited,
//! what the contract has settled, and the newest voucher signed.

/// More payees than this at once and the owner has a budget problem, not a
/// table problem; the table refuses rather than grows.
pub const MAX_CHANNELS: usize = 64;

/// A signature slot that holds nothing yet.
pub const NO_SIG: [u8; 65] = [0; 65];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Channel {
    pub id: [u8; 32],
    pub payee: [u8; 20],
    pub listing_id: [u8; 32],
    pub chain_id: u64,
    pub open_nonce: u64,
    pub unit: [u8; 32],
    /// What the contract holds for this channel, as the chain last said.
    pub deposit: u128,
    /// What the contract has already paid out, as the chain last said.
    pub settled: u128,
    /// The `cumulative` of the newest voucher. Never decreases.
    pub cumulative: u128,
    pub seq: u64,
    /// The channel's expiry, unix seconds. Every voucher carries exactly this,
    /// and none is signed at or past it.
    pub expiry: u64,
    /// The usage root of the newest voucher.
    pub usage_root: [u8; 32],
    /// The keyring's signature over the newest voucher, or NO_SIG while it
    /// has not come back. DRAIN hands out only signed vouchers.
    pub sig: [u8; 65],
    /// Settle reported more settled than this meter ever signed for: no
    /// further charge lands here, and ACCOUNT says why.
    pub frozen: bool,
    /// The cumulative the last settle() this meter signed carried for this
    /// channel, 0 before any: another settle needs a newer voucher (v1.4.7).
    pub carried: u128,
    /// Where close() sends what the channel holds back (v1.4.60), for ACCOUNT.
    pub reclaim_to: [u8; 20],
}
