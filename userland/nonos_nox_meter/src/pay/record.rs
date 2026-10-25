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

//! What OP_GET_COMMERCE said about a release, as PAY and FUNDED read it.

use alloc::vec::Vec;

/// What the market said about the release, as OP_GET_COMMERCE returns it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Offer {
    /// Both the publisher's release signature and the payee's binding hold.
    pub trusted: bool,
    pub listing_id: [u8; 32],
    pub release_hash: [u8; 32],
    pub chain_id: u64,
    pub token: Vec<u8>,
    /// The commerce block's canonical bytes.
    pub block: Vec<u8>,
}
