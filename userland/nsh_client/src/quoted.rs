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

//! What a document that passed says, for the client's own checks.

/// What a document that passed says, for the client's own checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Quoted {
    /// Where in the document its 32-byte challenge field starts. The
    /// client reads the challenge there itself, so an attester cannot
    /// hand back the challenge it was given in place of the document's.
    pub challenge_at: usize,
    /// The quote's P-256 attestation key, x then y.
    pub ak_public: [u8; 64],
    /// The quoted PCRs 0, 1, 2 and 7, in that order.
    pub firmware_pcrs: [[u8; 32]; 4],
}
