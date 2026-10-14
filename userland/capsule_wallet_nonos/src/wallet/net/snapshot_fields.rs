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

//! The fields an account refresh brings back, each None until the batch
//! reply holds it.

pub struct NoxStats {
    pub total: [u8; 32],
    pub rewards: [u8; 32],
    pub apr: Option<u64>,
}

#[derive(Default)]
pub struct Snapshot {
    pub eth_balance: Option<[u8; 32]>,
    pub nonce: Option<u64>,
    pub fee: Option<u64>,
    pub nox_balance: Option<[u8; 32]>,
    pub usdc_balance: Option<[u8; 32]>,
    pub claimable: Option<[u8; 32]>,
    pub positions: Option<u64>,
    /// ZeroState Passes the staking contract counts for this account. Taken
    /// from getStakeInfo rather than the NFT contract, since the boost is
    /// applied from what staking itself believes.
    pub passes: Option<u64>,
    pub stats: Option<NoxStats>,
    /// The newest block the host had when it answered.
    pub head: Option<u64>,
}
