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

//! A kept offer's id, and when it ends.

use nonos_marketplace_abi::Keccak;
use nonos_nox_config::OFFER_TTL;

use super::kept::KeptOffer;

impl KeptOffer {
    /// keccak256(cipherRoot | keyCommit), as ORDER_OFFER answers it.
    pub fn offer_id(&self, k: &impl Keccak) -> [u8; 32] {
        k.keccak256(&[self.offer.cipher_root, self.offer.key_commit].concat())
    }

    /// Past OFFER_TTL at `now`, or made in the future.
    pub fn expired(&self, now: u64) -> bool {
        self.made_at > now || now - self.made_at >= OFFER_TTL
    }
}
