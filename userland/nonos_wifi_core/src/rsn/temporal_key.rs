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
use crate::vault::{assert_not_copy, wipe};

/// Never Copy, Clone or Debug: every value is one owner's, wiped when it
/// is dropped, so no stray copy of the key outlives the link.
#[derive(PartialEq, Eq)]
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

    /// A second owned key, wiped on its own drop like the first.
    pub(crate) fn again(&self) -> Self {
        match self {
            TemporalKey::Ccmp128(k) => TemporalKey::Ccmp128(*k),
            TemporalKey::Gcmp256(k) => TemporalKey::Gcmp256(*k),
        }
    }

    pub fn bytes(&self) -> &[u8] {
        match self {
            TemporalKey::Ccmp128(k) => k,
            TemporalKey::Gcmp256(k) => k,
        }
    }
}

assert_not_copy!(TemporalKey);

impl Drop for TemporalKey {
    fn drop(&mut self) {
        match self {
            TemporalKey::Ccmp128(k) => wipe(k),
            TemporalKey::Gcmp256(k) => wipe(k),
        }
    }
}
