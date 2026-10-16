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

//! The escrow offer's constants (plan/29, v1.4.29; the owner may change
//! them): the least Equi-X effort an agent takes, how long an offer lives,
//! and how many the meter keeps.

/// The least effort a FETCH 1 request's proof of work carries (Tor's rule:
/// BLAKE2b-32 of challenge and solution, times effort, fits a u32).
pub const ESCROW_POW_EFFORT: u32 = 32;
/// An untaken offer is dropped after this long, and the agent forgets a
/// solution it has seen only after it.
pub const OFFER_TTL: u64 = 24 * 3_600;
/// The offers the meter keeps at once; a fifth evicts the oldest.
pub const MAX_OFFERS: usize = 4;
