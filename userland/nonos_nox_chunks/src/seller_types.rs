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

//! The seller's side of an order, and why it would not deliver.

/// The seller's own side of an order: its release and itself.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Mine {
    pub listing_id: [u8; 32],
    pub release_id: [u8; 32],
    pub chunk_root: [u8; 32],
    pub content_len: u64,
    pub price: u128,
    pub payee: [u8; 20],
    pub bond_id: [u8; 32],
    pub nox: [u8; 20],
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TermsWrong {
    /// Another listing, release, chunk tree or length.
    NotMyRelease,
    /// cipherRoot or keyCommit is not this ciphertext's or this key's.
    NotMyCipher,
    /// Another token, payee or bond, or less than the price.
    NotMyPrice,
    /// The deadline or the dispute window is outside the rules.
    BadTimes,
}
