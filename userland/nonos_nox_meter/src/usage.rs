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

//! One charge, as the OZ standard-tree leaf the settlement contract can open
//! in a dispute: keccak256(bytes.concat(keccak256(abi.encode(channelId, seq,
//! unit, qty, price, callId)))).

use nonos_marketplace_abi::Keccak;

use crate::words::{u128_word, u64_word};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Charge {
    pub channel_id: [u8; 32],
    pub seq: u64,
    pub unit: [u8; 32],
    pub qty: u64,
    /// The unit price the charge was made at.
    pub price: u128,
    pub call_id: [u8; 32],
}

impl Charge {
    pub fn leaf(&self, k: &impl Keccak) -> [u8; 32] {
        let mut pre = [0u8; 192];
        pre[..32].copy_from_slice(&self.channel_id);
        pre[32..64].copy_from_slice(&u64_word(self.seq));
        pre[64..96].copy_from_slice(&self.unit);
        pre[96..128].copy_from_slice(&u64_word(self.qty));
        pre[128..160].copy_from_slice(&u128_word(self.price));
        pre[160..].copy_from_slice(&self.call_id);
        k.keccak256(&k.keccak256(&pre))
    }
}
