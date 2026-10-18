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

//! Why a record was not made, read or kept.

use crate::keys::KeysError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BondError {
    /// Not the length the record's kind fixes.
    Length,
    /// A version this crate does not know.
    Version,
    /// A field outside what its definition allows, or a reserved octet not
    /// zero.
    Invalid,
    /// The peer left no identity by which it could be found again.
    NoIdentity,
    /// The book holds `MAX_BONDS` already.
    Full,
    /// Where records are kept refused or failed.
    Keys(KeysError),
}

impl From<KeysError> for BondError {
    fn from(e: KeysError) -> Self {
        Self::Keys(e)
    }
}
