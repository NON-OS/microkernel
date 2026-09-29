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

//! The running SHA-1 state, and why it has to be cloneable.

pub const BLOCK_BYTES: usize = 64;
pub const DIGEST_BYTES: usize = 20;

/*
 * The relay digest is one hash that runs for the life of a hop, not one
 * hash per cell. Every cell in a direction is fed into it in order, and
 * the four bytes that travel in the cell are the leading bytes of the
 * digest *at that point*, taken without ending the hash. So this type is
 * `Clone`: reading a value means cloning the state and finishing the
 * clone, which leaves the original free to absorb the next cell.
 */
#[derive(Clone)]
pub struct Sha1 {
    pub(super) state: [u32; 5],
    pub(super) block: [u8; BLOCK_BYTES],
    pub(super) buffered: usize,
    pub(super) total: u64,
}

impl Default for Sha1 {
    fn default() -> Self {
        Self::new()
    }
}

impl Sha1 {
    pub fn new() -> Self {
        Self {
            state: [0x6745_2301, 0xefcd_ab89, 0x98ba_dcfe, 0x1032_5476, 0xc3d2_e1f0],
            block: [0u8; BLOCK_BYTES],
            buffered: 0,
            total: 0,
        }
    }
}
