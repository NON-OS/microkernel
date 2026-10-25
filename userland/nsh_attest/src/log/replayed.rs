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

//! What a replay gives.

use alloc::vec::Vec;

use super::{Admitted, PCRS};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Replayed {
    pub pcrs: [[u8; 32]; PCRS],
    /// The last application the firmware measured into PCR 4.
    pub last_application: Option<[u8; 32]>,
    /// Every application measured into PCR 4, in log order.
    pub applications: Vec<[u8; 32]>,
    /// Events after the header.
    pub events: usize,
    /// The capsules PCR 14 records, in the order they were admitted.
    pub capsules: Vec<Capsule>,
}

/// One admitted capsule: its certificate's digest, the digest of what
/// verified it, and what admitted it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Capsule {
    pub cert: [u8; 32],
    pub verifier: [u8; 32],
    pub admitted: Admitted,
}
