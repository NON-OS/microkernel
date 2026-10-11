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

//! One field of a struct the keyring will sign, and the shape its 32-byte
//! word must have. A word that does not fit its type would hash to a value
//! no contract decodes the same way, so it is refused rather than signed.
//! Names live in the type string; wallet_proofs checks the two agree.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Bytes32,
    Address,
    /// uintN, with N the bit width.
    Uint(u16),
}

#[derive(Clone, Copy, Debug)]
pub struct Field {
    pub kind: Kind,
    /// The field must name the key that signs: a voucher's payer is the
    /// meter, never someone the meter writes in.
    pub is_signer: bool,
}

/// Whether `w` is a canonical EIP-712 encoding of `kind`: big-endian, with
/// every bit above the type's width zero.
pub fn well_formed(kind: Kind, w: &[u8; 32]) -> bool {
    let zero_bytes = match kind {
        Kind::Bytes32 => 0,
        Kind::Address => 12,
        Kind::Uint(bits) if bits % 8 == 0 && (8..=256).contains(&bits) => 32 - bits as usize / 8,
        Kind::Uint(_) => return false,
    };
    w[..zero_bytes].iter().all(|&b| b == 0)
}
