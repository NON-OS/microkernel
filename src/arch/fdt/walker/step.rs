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

use crate::arch::fdt::endian::be_u32;
use crate::arch::fdt::error::FdtError;
use crate::arch::fdt::limits::MAX_DEPTH;
use crate::arch::fdt::tokens::{FDT_BEGIN_NODE, FDT_END, FDT_END_NODE, FDT_NOP, FDT_PROP};

use super::event::Event;
use super::pass::Walker;
use super::read::{node_name, property};

impl<'a> Walker<'a> {
    /// The next node boundary or property, `None` once FDT_END closes a
    /// balanced tree. Running off the block without FDT_END is an error, not
    /// an end: a truncated blob must not read as a complete one.
    pub fn next_event(&mut self) -> Result<Option<Event<'a>>, FdtError> {
        while !self.done {
            if self.cursor >= self.structure.len() {
                return Err(FdtError::MissingEnd);
            }
            if !self.cursor.is_multiple_of(4) {
                return Err(FdtError::UnalignedToken);
            }
            let tok = be_u32(self.structure, self.cursor)?;
            self.cursor += 4;
            match tok {
                FDT_BEGIN_NODE => {
                    if self.depth >= MAX_DEPTH {
                        return Err(FdtError::TooDeep);
                    }
                    let (name, body) = node_name(self.structure, self.cursor)?;
                    self.cursor = body;
                    self.depth += 1;
                    return Ok(Some(Event::BeginNode { name, body }));
                }
                FDT_END_NODE => {
                    self.depth = self.depth.checked_sub(1).ok_or(FdtError::Unbalanced)?;
                    return Ok(Some(Event::EndNode));
                }
                FDT_PROP => {
                    let (prop, next) = property(self.structure, self.strings, self.cursor)?;
                    self.cursor = next;
                    return Ok(Some(Event::Property(prop)));
                }
                FDT_NOP => {}
                FDT_END if self.depth == 0 => self.done = true,
                FDT_END => return Err(FdtError::Unbalanced),
                _ => return Err(FdtError::UnknownToken),
            }
        }
        Ok(None)
    }
}
