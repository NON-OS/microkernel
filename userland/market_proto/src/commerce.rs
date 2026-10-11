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

//! What a listing sells and how far it can be trusted, as `OP_GET_COMMERCE`
//! returns it for one release (an empty release id is the default):
//!
//!   u8 commerce verdict | u8 binding verdict | listing_id[32] | release_id[32]
//!   | release_hash[32] | u64 chain_id | lp token contract | lp commerce block
//!
//! The ids are the on-chain ones (plan/23); `release_hash` is zero for a
//! schema 2 release, and so is an empty commerce block. The block is the
//! abi's canonical bytes, read with `nonos_marketplace_abi`'s decoder.

use alloc::vec::Vec;

/// The release signed the listed commerce block.
pub const COMMERCE_MATCHES: u8 = 0;
/// The payee's PublisherBinding recovers to the payee under the bond domain.
pub const BINDING_VALID: u8 = 0;

pub struct Commerce {
    pub commerce_verdict: u8,
    pub binding_verdict: u8,
    pub listing_id: [u8; 32],
    pub release_id: [u8; 32],
    pub release_hash: [u8; 32],
    pub chain_id: u64,
    pub token_contract: Vec<u8>,
    pub block: Vec<u8>,
}

impl Commerce {
    /// Both the publisher and the payee stand behind price and payee.
    pub fn trusted(&self) -> bool {
        self.commerce_verdict == COMMERCE_MATCHES && self.binding_verdict == BINDING_VALID
    }
}
