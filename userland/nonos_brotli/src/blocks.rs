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

use crate::bits::Bits;
use crate::error::Error;
use crate::header::read_count;
use crate::huff::Code;
use crate::lengths::block_count;
use crate::prefix::read_code;

/// The block type state of one category: literals, commands or distances.
pub(crate) struct Blocks {
    pub(crate) types: usize,
    kind: usize,
    prev: usize,
    left: u32,
    codes: Option<(Code, Code)>,
}

impl Blocks {
    pub(crate) fn read(b: &mut Bits) -> Result<Blocks, Error> {
        let types = read_count(b)?;
        let (codes, left) = match types {
            1 => (None, 0),
            _ => {
                let (kinds, counts) = (read_code(b, types + 2)?, read_code(b, 26)?);
                let left = block_count(b, &counts)?;
                (Some((kinds, counts)), left)
            }
        };
        Ok(Blocks { types, kind: 0, prev: 1, left, codes })
    }

    /// The block type of the next symbol, switching types once the
    /// current block is spent.
    pub(crate) fn next(&mut self, b: &mut Bits) -> Result<usize, Error> {
        let Some((kinds, counts)) = &self.codes else {
            return Ok(0);
        };
        if self.left == 0 {
            let next = match kinds.read(b)? as usize {
                0 => self.prev,
                1 => self.kind + 1,
                s => s - 2,
            } % self.types;
            (self.prev, self.kind) = (self.kind, next);
            self.left = block_count(b, counts)?;
        }
        self.left -= 1;
        Ok(self.kind)
    }
}
