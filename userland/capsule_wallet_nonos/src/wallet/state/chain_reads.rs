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
//! What the chain last said of this account on the picked network: its
//! ETH and USDC, its next nonce, and the fee. Each with whether it came.

#[derive(Default)]
pub struct ChainReads {
    pub balance_ready: bool,
    pub balance_wei: [u8; 32],
    /// USDC held on the picked network, in its six-place units.
    pub usdc_ready: bool,
    pub usdc_units: [u8; 32],
    pub nonce_ready: bool,
    pub live_nonce: u64,
    pub fee_ready: bool,
    pub fee_wei: u64,
}
