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

//! Spoiling one half of a hybrid signature, to show both halves are
//! checked.

/// Which half of a hybrid signature a test spoils, if any.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Spoil {
    #[default]
    None,
    Ed,
    Ml,
}

impl Spoil {
    /// `sig` is Ed25519 then ML-DSA-65; one bit of the chosen half flips.
    pub fn apply(self, sig: &mut [u8]) {
        match self {
            Spoil::None => {}
            Spoil::Ed => sig[10] ^= 1,
            Spoil::Ml => sig[64 + 100] ^= 1,
        }
    }
}
