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

use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// The witness proves no statement: a success outside the set, or a word
    /// out of range. Nothing was proven.
    Witness,
    /// The circuit could not be laid out for this statement.
    Shape,
    /// Fewer than the blinding seed's bytes of entropy.
    Entropy,
    /// The prover produced nothing.
    Prover,
    /// The proof does not verify.
    NotVerified(String),
    /// Verified, but the zero-knowledge certificate was not reached: prove
    /// again with fresh entropy. Nothing was emitted.
    Rank(String),
}
