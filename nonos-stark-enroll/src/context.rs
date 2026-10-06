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

//! The contexts the gates build, byte for byte, and the tree's shape.

pub const POLICY_EPOCH: u64 = 1;
pub const BOOT_EPOCH: u64 = 1;
pub const DEPTH: usize = 8;
pub const LEAVES: usize = 1 << DEPTH;

/// The capsule spawn gate's context, built by the one function every gate uses.
pub fn capsule_context(image_hash: &[u8; 32], caps: u64) -> Vec<u8> {
    nonos_attest_path::capsule_context(image_hash, caps, POLICY_EPOCH).to_vec()
}

/// A kernel's or a bootloader's context at the boot epoch, the same way.
pub fn kernel_context(image_hash: &[u8; 32]) -> Vec<u8> {
    nonos_attest_path::boot_context(image_hash, BOOT_EPOCH).to_vec()
}
