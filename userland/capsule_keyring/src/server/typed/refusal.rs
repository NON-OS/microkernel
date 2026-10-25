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

//! Why a typed-data request was not signed. The handler turns each into an
//! errno; the reason is kept apart so the pure checks can be tested by it.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refusal {
    /// The request is not the shape the wire format says.
    Malformed,
    /// No entry has that number.
    UnknownEntry,
    /// The caller owns none of the entry's services, or the pid it wrote
    /// is not its own.
    Caller,
    /// Not mainnet or Sepolia.
    Chain,
    /// The entry's contract is not deployed on that chain.
    NotDeployed,
    /// Field `n` is not a canonical encoding of its type.
    Field(usize),
    /// A field that must name the signer names someone else.
    NotSigner,
}
