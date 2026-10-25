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

//! What a tradable asset is.

/// A tradable asset: what to call it, how to divide it, where it lives.
#[derive(Clone, Copy)]
pub struct Token {
    pub symbol: &'static str,
    /// Contract address, or all zero for the chain's own coin.
    pub address: [u8; 20],
    pub decimals: u8,
}

impl Token {
    /// Whether this is the chain's own coin rather than a contract.
    ///
    /// A native coin needs no approval before a swap, which is the one place
    /// the distinction changes what the screen asks the reader to do.
    pub fn is_native(&self) -> bool {
        self.address == [0u8; 20]
    }
}
