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

//! The randomness one join consumes, drawn whole before it begins.

use super::*;

/// Fresh randomness for one join. Never derived from anything sent on the air:
/// the SNonce goes out in message 2, so SAE's secrets are drawn separately.
/// Wiped when it goes (`wipe.rs`).
pub struct Entropy {
    pub snonce: [u8; 32],
    pub sae: [u8; SAE_ENTROPY],
    /// The seed pairwise rekeys draw their SNonces from (`Supplicant`).
    pub rekey: [u8; 32],
}

impl Entropy {
    /// Draw a join's randomness through `fill`, which fills a buffer whole or
    /// says it could not. None when any draw fails, so a join is refused
    /// before it begins and never runs on zeros.
    pub fn draw(mut fill: impl FnMut(&mut [u8]) -> bool) -> Option<Entropy> {
        let mut e = Entropy { snonce: [0; 32], sae: [0; SAE_ENTROPY], rekey: [0; 32] };
        (fill(&mut e.snonce) && fill(&mut e.sae) && fill(&mut e.rekey)).then_some(e)
    }
}
