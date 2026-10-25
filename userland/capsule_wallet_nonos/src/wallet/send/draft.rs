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
use alloc::vec::Vec;

use super::{ASSET_ETH, ASSET_NOX, ASSET_USDC};
use crate::wallet::act::Purpose;
use crate::wallet::chain;
use crate::wallet::state::State;

#[path = "draft_finish.rs"]
mod draft_finish;
#[path = "draft_held.rs"]
mod draft_held;
#[path = "draft_plan.rs"]
mod draft_plan;
pub use draft_finish::finish;
pub use draft_held::held;
use draft_held::transfer_data;
pub use draft_plan::plan;

/// What the review shows and the confirm signs, field for field.
#[derive(Clone)]
pub struct Draft {
    /// A payment, or one of the staking transactions.
    pub purpose: Purpose,
    pub asset: u8,
    pub recipient: [u8; 20],
    pub amount: u128,
    /// The transaction's own target: the recipient for ETH, the token
    /// contract for a token.
    pub to: [u8; 20],
    pub value: u128,
    pub data: Vec<u8>,
    pub gas: u64,
    pub nonce: u64,
    pub tip: u128,
    pub max_fee: u128,
    pub chain_id: u64,
}

impl Draft {
    /// The most the network fee can come to: every unit of gas at the cap.
    pub fn fee_cap_wei(&self) -> u128 {
        (self.gas as u128).saturating_mul(self.max_fee)
    }
}

/// The payment typed on the form, checked against what is held, before the
/// network is asked for the nonce, the fee and the gas.
#[derive(Clone)]
pub struct Plan {
    pub purpose: Purpose,
    pub asset: u8,
    pub recipient: [u8; 20],
    pub amount: u128,
    pub to: [u8; 20],
    pub value: u128,
    pub data: Vec<u8>,
    pub chain_id: u64,
}
