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

//! The Voucher struct of plan/20, as the nine words the keyring signs under
//! entry 1 of its typed-data table.

use nonos_marketplace_abi::Keccak;
use nonos_nox_config::VOUCHER;

use crate::words::{addr_word, u128_word, u64_word};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Voucher {
    pub channel_id: [u8; 32],
    /// The meter's own address: the keyring refuses any other.
    pub payer: [u8; 20],
    pub payee: [u8; 20],
    pub listing_id: [u8; 32],
    pub unit: [u8; 32],
    pub cumulative: u128,
    pub seq: u64,
    pub expiry: u64,
    pub usage_root: [u8; 32],
}

impl Voucher {
    /// In type-string order, each canonical for its type.
    pub fn words(&self) -> [[u8; 32]; 9] {
        [
            self.channel_id,
            addr_word(&self.payer),
            addr_word(&self.payee),
            self.listing_id,
            self.unit,
            u128_word(self.cumulative),
            u64_word(self.seq),
            u64_word(self.expiry),
            self.usage_root,
        ]
    }

    /// keccak256(typehash | words): what the keyring's structHash must be.
    pub fn struct_hash(&self, k: &impl Keccak) -> [u8; 32] {
        let mut pre = [0u8; 320];
        pre[..32].copy_from_slice(&VOUCHER.typehash);
        for (i, w) in self.words().iter().enumerate() {
            pre[32 * (i + 1)..32 * (i + 2)].copy_from_slice(w);
        }
        k.keccak256(&pre)
    }
}
