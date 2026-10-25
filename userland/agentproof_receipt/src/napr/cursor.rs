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
//! A receipt read, each field held to its length and nothing left over.

//! A reader's place in a receipt, and one length-prefixed field taken.

use super::read::Fault;

pub(super) struct Cursor<'a> {
    pub(super) b: &'a [u8],
    pub(super) at: usize,
}

impl<'a> Cursor<'a> {
    pub(super) fn field(
        &mut self,
        name: &'static str,
        min: usize,
        max: usize,
    ) -> Result<&'a [u8], Fault> {
        let len = self.b.get(self.at..self.at + 4).ok_or(Fault::Field(name))?;
        let len = u32::from_be_bytes([len[0], len[1], len[2], len[3]]) as usize;
        if len < min || len > max {
            return Err(Fault::Field(name));
        }
        let start = self.at + 4;
        let v = self.b.get(start..start + len).ok_or(Fault::Field(name))?;
        self.at = start + len;
        Ok(v)
    }
}
