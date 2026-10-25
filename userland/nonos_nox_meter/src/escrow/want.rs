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

//! What an escrow request is for, and what the meter keeps of it.

use alloc::vec::Vec;

use nonos_settle_proto::Escrow;

/// What the request is for: the chain, the release's https endpoint, and its keys.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Want {
    pub chain_id: u64,
    pub url: Vec<u8>,
    pub listing_key: [u8; 32],
    pub release_key: [u8; 32],
}

pub struct Asked {
    pub buyer_sk: [u8; 32],
    pub request: Escrow,
}
