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
use nonos_marketplace_abi::Keccak;
use nonos_nox_config::CHANNEL_TAG;

use crate::words::{addr_word, u64_word};

/// What names one channel on chain. `open_nonce` is the payer's counter the
/// settlement contract read on open, from the ChannelOpened event, so a
/// closed and reopened channel never shares an id with the old one.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ChannelKey {
    pub chain_id: u64,
    pub settlement: [u8; 20],
    pub payer: [u8; 20],
    pub payee: [u8; 20],
    pub listing_id: [u8; 32],
    pub open_nonce: u64,
}

impl ChannelKey {
    /// keccak256(abi.encode(CHANNEL_TAG, chainId, settlement, payer, payee,
    /// listingId, openNonce)), plan/21.
    pub fn id(&self, k: &impl Keccak) -> [u8; 32] {
        let mut pre = [0u8; 224];
        pre[..32].copy_from_slice(&CHANNEL_TAG);
        pre[32..64].copy_from_slice(&u64_word(self.chain_id));
        pre[64..96].copy_from_slice(&addr_word(&self.settlement));
        pre[96..128].copy_from_slice(&addr_word(&self.payer));
        pre[128..160].copy_from_slice(&addr_word(&self.payee));
        pre[160..192].copy_from_slice(&self.listing_id);
        pre[192..].copy_from_slice(&u64_word(self.open_nonce));
        k.keccak256(&pre)
    }
}
