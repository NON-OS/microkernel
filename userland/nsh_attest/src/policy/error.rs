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

//! Why a policy was refused before it was used.

/// Why a policy was refused before it was used.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyError {
    /// The fleet id is all zeros.
    Fleet,
    /// No authority, more than `MAX_AUTHORITIES`, or one listed twice.
    Authorities,
    /// No release, or more than `MAX_RELEASES`.
    Releases,
    /// A release with a zero PCR 8 or 9, loader, front or network
    /// measurement.
    ZeroValue,
    /// A release whose network set lists a measurement twice, lists its
    /// front, or holds more than `MAX_NETWORK`.
    Network,
    /// Two releases with the same PCR 9: a kernel would match both.
    SharedPcr9,
    /// More than `MAX_REVOKED` revoked enrolments.
    Revoked,
}
