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

//! An ID mapping (20 bytes): input base, the number of IDs less one,
//! output base, the output node as an offset into the IORT, and flags
//! (bit 0: a single mapping, the node's own ID, whatever the input).

use super::super::le::u32_at;

pub const LEN: usize = 20;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Mapping {
    pub input_base: u32,
    /// The field as written: the number of IDs in the range, less one.
    pub count_less_one: u32,
    pub output_base: u32,
    pub output_ref: u32,
    pub single: bool,
}

impl Mapping {
    pub fn read(m: &[u8]) -> Option<Mapping> {
        Some(Mapping {
            input_base: u32_at(m, 0)?,
            count_less_one: u32_at(m, 4)?,
            output_base: u32_at(m, 8)?,
            output_ref: u32_at(m, 12)?,
            single: u32_at(m, 16)? & 1 != 0,
        })
    }

    /// The output ID for `id`, when this range holds it. A single mapping
    /// holds no range: it is the node's own ID, asked for separately.
    pub fn map(&self, id: u32) -> Option<u32> {
        if self.single {
            return None;
        }
        let off = id.checked_sub(self.input_base)?;
        (off <= self.count_less_one).then(|| self.output_base.checked_add(off)).flatten()
    }
}
