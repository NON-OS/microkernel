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

//! The memory reservation block: ranges the kernel must never hand out, such
//! as the Pi firmware's spin tables at the bottom of RAM.

use super::endian::be_u64;
use super::error::FdtError;
use super::parser::Fdt;

impl<'a> Fdt<'a> {
    /// Every reserved `(address, size)` up to the all-zero terminator. A map
    /// that runs into the next block without one is reported, since dropping
    /// the tail would hand out memory the firmware still owns.
    pub fn reservations(&self) -> impl Iterator<Item = Result<(u64, u64), FdtError>> + 'a {
        let map = self.rsvmap;
        let mut offset = 0usize;
        let mut done = false;
        core::iter::from_fn(move || {
            if done {
                return None;
            }
            let entry = be_u64(map, offset).and_then(|a| Ok((a, be_u64(map, offset + 8)?)));
            offset += 16;
            match entry {
                Ok((0, 0)) => {
                    done = true;
                    None
                }
                Ok(pair) => Some(Ok(pair)),
                Err(_) => {
                    done = true;
                    Some(Err(FdtError::MissingEnd))
                }
            }
        })
    }
}
