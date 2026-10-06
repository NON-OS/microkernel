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

//! What only the device holds. None of it leaves in a proof.

use alloc::vec::Vec;
use stark_proofs::crypto::stark::air::RATE;
use stark_proofs::crypto::stark::field::Fp;

/// A path from a leaf to a root: a sibling per level, and whether the running
/// node is the right child there.
#[derive(Clone, Debug)]
pub struct Path {
    pub siblings: Vec<[Fp; RATE]>,
    pub right: Vec<bool>,
}

/// An enrolled slot: its context digest and its path.
#[derive(Clone, Debug)]
pub struct Slot {
    pub digest: [u8; 32],
    pub path: Path,
}

#[derive(Clone, Debug)]
pub struct Witness {
    /// The device secret: four field elements drawn by rejection, never reduced.
    pub secret: [Fp; RATE],
    pub bootloader: Slot,
    pub kernel: Slot,
    /// The path of `commit(secret)` in the registry.
    pub device: Path,
}
