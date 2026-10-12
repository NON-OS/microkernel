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

//! The data ciphers this station runs (IEEE Std 802.11-2020, Table 9-149):
//! CCMP-128, which every RSN link has, and GCMP-256 where the radio offers
//! it. A cipher fixes its temporal key's length (12.7.2: 16 octets for
//! CCMP-128, 32 for GCMP-256), so the length is always read from the cipher
//! and never kept beside it.

use super::suite::{CIPHER_CCMP, CIPHER_GCMP_256};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cipher {
    Ccmp128,
    Gcmp256,
}

impl Cipher {
    /// The suite type octet after the 00-0F-AC OUI.
    pub const fn suite_type(self) -> u8 {
        match self {
            Cipher::Ccmp128 => 4,
            Cipher::Gcmp256 => 9,
        }
    }

    /// The temporal key's length in octets.
    pub const fn tk_len(self) -> usize {
        match self {
            Cipher::Ccmp128 => 16,
            Cipher::Gcmp256 => 32,
        }
    }

    /// The cipher a suite selector names, if it is one this station runs.
    pub fn from_selector(s: u32) -> Option<Self> {
        match s {
            CIPHER_CCMP => Some(Cipher::Ccmp128),
            CIPHER_GCMP_256 => Some(Cipher::Gcmp256),
            _ => None,
        }
    }
}

/// The ciphers a link can run beyond CCMP-128, which every link runs.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Ciphers {
    pub gcmp256: bool,
}

impl Ciphers {
    /// CCMP-128 alone: what a radio reports until it says otherwise.
    pub const CCMP_128_ONLY: Ciphers = Ciphers { gcmp256: false };

    pub const fn has(self, c: Cipher) -> bool {
        match c {
            Cipher::Ccmp128 => true,
            Cipher::Gcmp256 => self.gcmp256,
        }
    }
}
