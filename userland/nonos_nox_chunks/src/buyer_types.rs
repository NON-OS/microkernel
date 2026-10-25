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

//! The two sides of a buyer's order: what the sale fixes, and what the
//! buyer itself chooses and draws.

/* Rule 5's half of Buyer: its wipe, its Drop and its Debug. */
#[path = "buyer_secret.rs"]
mod secret;

/// What the listing, its release and the checked offer fix.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Sale {
    pub listing_id: [u8; 32],
    pub release_id: [u8; 32],
    pub bond_id: [u8; 32],
    pub payee: [u8; 20],
    pub amount: u128,
    pub chunk_root: [u8; 32],
    pub content_len: u64,
    pub cipher_root: [u8; 32],
    pub key_commit: [u8; 32],
}

/// The buyer's deadline, its committed refund address, and two fresh salts.
/// The salts are secret (rule 5): its Debug names them redacted, it is not
/// Copy, and dropping it wipes them.
#[derive(Clone, PartialEq, Eq)]
pub struct Buyer {
    pub deadline: u64,
    pub refund_to: [u8; 20],
    pub refund_salt: [u8; 32],
    pub salt: [u8; 32],
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OrderWrong {
    /// Nothing to pay, nothing to deliver, or a zero refundTo.
    Empty,
    /// The deadline is outside the window.
    Deadline,
}
