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

//! A temporal key with the cipher it is for; the variant fixes the length,
//! so a key cannot be run under the wrong cipher.

use super::cipher::Cipher;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TemporalKey {
    Ccmp128([u8; 16]),
    Gcmp256([u8; 32]),
}

impl TemporalKey {
    /// The key for `cipher` from `key`, if `key` is that cipher's length.
    pub fn new(cipher: Cipher, key: &[u8]) -> Option<Self> {
        Some(match cipher {
            Cipher::Ccmp128 => TemporalKey::Ccmp128(key.try_into().ok()?),
            Cipher::Gcmp256 => TemporalKey::Gcmp256(key.try_into().ok()?),
        })
    }

    pub fn cipher(&self) -> Cipher {
        match self {
            TemporalKey::Ccmp128(_) => Cipher::Ccmp128,
            TemporalKey::Gcmp256(_) => Cipher::Gcmp256,
        }
    }

    pub fn bytes(&self) -> &[u8] {
        match self {
            TemporalKey::Ccmp128(k) => k,
            TemporalKey::Gcmp256(k) => k,
        }
    }
}
