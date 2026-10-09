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

use crate::field::Fp;

/// State width in field elements.
pub const WIDTH: usize = 8;
/// The lanes a digest occupies.
pub const RATE: usize = 4;
/// Full rounds per permutation, the attestation tree's `2^5`.
pub(crate) const ROUNDS: usize = 32;
/// The round-constant rule's domain, the same string the STARK tree uses.
pub(crate) const RC_DOMAIN: &[u8] = b"NONOS-POSEIDON-GOLDILOCKS-RC";

/// A tree node, a leaf or a root: four field elements.
pub type Digest = [Fp; RATE];

/// The deepest path a trailer may claim. The policy tree is depth 8; the bound
/// only keeps a hostile depth byte from asking for more work than any real
/// tree could need.
pub const MAX_DEPTH: usize = 32;
