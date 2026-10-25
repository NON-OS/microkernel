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

use crate::budget::Budget;
use crate::channel::Channel;

/// Everything the meter keeps on disk. Every change makes a new value, which
/// is written before anything is signed from it.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct MeterState {
    pub budget: Budget,
    pub channels: Vec<Channel>,
    /// The chain and nonce of the last settle() this meter signed: the one
    /// nonce at which it may sign a replacement (a fee bump).
    pub last_settle: Option<(u64, u64)>,
    /// When the meter last signed the same vouchers again because the last
    /// settle's nonce went to another transaction, unix seconds (v1.4.12).
    pub last_retry: u64,
}

impl MeterState {
    pub fn channel(&self, payee: &[u8; 20], listing: &[u8; 32], chain: u64) -> Option<&Channel> {
        self.channels
            .iter()
            .find(|c| &c.payee == payee && &c.listing_id == listing && c.chain_id == chain)
    }
}
