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

/*
 * What the ReleaseAttest log holds and answers.
 */

/// One ReleaseAttest's fields, as entry 3 signs them, with its chain.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Attest {
    pub chain_id: u64,
    pub listing_id: [u8; 32],
    pub release_id: [u8; 32],
    pub bond_id: [u8; 32],
    pub chunk_root: [u8; 32],
    pub content_len: u64,
    pub package_hash: [u8; 32],
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Record {
    pub attest: Attest,
    pub sig: [u8; 65],
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Decision {
    /// Nothing signed for this release yet: sign, then record it.
    Sign,
    /// This very attest was signed before: here is its signature.
    Again([u8; 65]),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Refusal {
    /// A different attest for this chain, listing and release was signed.
    Conflict,
    /// The log holds as many as it can.
    Full,
}
