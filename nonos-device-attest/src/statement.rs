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

//! What the verifier holds, and the order it enters the transcript.

use alloc::vec::Vec;
use stark_proofs::crypto::stark::air::RATE;
use stark_proofs::crypto::stark::field::Fp;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Statement {
    /// The bootloader tree's root, published with the release.
    pub boot_root: [Fp; RATE],
    /// The kernel tree's root, the one the bootloader folds to.
    pub kernel_root: [Fp; RATE],
    /// The device registry's root.
    pub device_root: [Fp; RATE],
    /// The registry's depth, which fixes the circuit's shape.
    pub device_depth: usize,
    /// The verifier's scope.
    pub scope: [Fp; 2],
    /// The verifier's context, a nonce against replay.
    pub context: [Fp; RATE],
    /// The device's tag in this scope.
    pub tag: [Fp; RATE],
}

impl Statement {
    /// Every public value, absorbed before the first commitment, so a proof is
    /// bound to all of them and to the context in particular.
    pub fn publics(&self) -> Vec<Fp> {
        let mut p = Vec::with_capacity(4 * RATE + 2 + RATE + 1);
        p.extend_from_slice(&self.boot_root);
        p.extend_from_slice(&self.kernel_root);
        p.extend_from_slice(&self.device_root);
        p.push(Fp::from_u64(self.device_depth as u64));
        p.extend_from_slice(&self.scope);
        p.extend_from_slice(&self.context);
        p.extend_from_slice(&self.tag);
        p
    }
}
