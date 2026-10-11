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

//! What the agent serves, why it refuses, and what one answer leaves behind.

/// The release this agent sells, its keys and its bytes.
pub struct Served {
    pub listing_key: [u8; 32],
    pub release_key: [u8; 32],
    pub content: Vec<u8>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Refusal {
    Body,
    Chain,
    NotServed,
    Solution,
    Seen,
    /// buyer_pk makes no shared secret, or the content is empty.
    Seal,
}

pub struct Answered {
    pub sealed: Vec<u8>,
    pub k: [u8; 32],
    pub key_commit: [u8; 32],
    pub cipher_root: [u8; 32],
    pub buyer_pk: [u8; 32],
}
