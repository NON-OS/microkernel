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

use alloc::vec::Vec;

use crate::image::types::DecodeError;

use super::lzw_dict::{Dict, MAX_CODES};

impl Dict {
    /* Push the string for `code` onto `stack` last byte first and return its
     * first byte. A code not yet in the table is the previous string plus
     * its own first byte (the KwKwK case). */
    pub fn expand(
        &self,
        code: u16,
        old: u16,
        first: u8,
        stack: &mut Vec<u8>,
    ) -> Result<u8, DecodeError> {
        let mut cur = code;
        if code >= self.free {
            stack.push(first);
            cur = old;
        }
        while cur >= self.clear {
            let c = cur as usize;
            if c >= MAX_CODES || stack.len() > MAX_CODES {
                return Err(DecodeError::BadMagic);
            }
            stack.push(self.suffix[c]);
            cur = self.prefix[c];
        }
        let head = self.suffix[cur as usize];
        stack.push(head);
        Ok(head)
    }
}
