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

//! The m' blocks one derivation fills, lent by the heap in 1 MiB pieces so
//! no single allocation has to find the whole of it in one run, and wiped
//! before they are handed back.

use alloc::vec::Vec;

use super::block::Block;
use super::params::Argon2Error;
use super::wipe::wipe;

/// Blocks per piece: 1024 blocks of 1 KiB.
const PIECE: usize = 1024;

pub(super) struct Memory {
    pieces: Vec<Vec<Block>>,
}

impl Memory {
    /// `blocks` zeroed blocks, or `NoMemory` with nothing kept.
    pub(super) fn new(blocks: usize) -> Result<Self, Argon2Error> {
        let mut pieces = Vec::new();
        pieces.try_reserve_exact(blocks.div_ceil(PIECE)).map_err(|_| Argon2Error::NoMemory)?;
        let mut left = blocks;
        while left > 0 {
            let n = left.min(PIECE);
            let mut piece = Vec::new();
            piece.try_reserve_exact(n).map_err(|_| Argon2Error::NoMemory)?;
            piece.resize(n, [0u64; 128]);
            pieces.push(piece);
            left -= n;
        }
        Ok(Self { pieces })
    }

    pub(super) fn at(&self, i: usize) -> &Block {
        &self.pieces[i / PIECE][i % PIECE]
    }

    pub(super) fn at_mut(&mut self, i: usize) -> &mut Block {
        &mut self.pieces[i / PIECE][i % PIECE]
    }
}

impl Drop for Memory {
    fn drop(&mut self) {
        for piece in self.pieces.iter_mut() {
            for block in piece.iter_mut() {
                wipe(block);
            }
        }
    }
}
