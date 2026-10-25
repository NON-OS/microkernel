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

//! The parser's state between items, and the items that only move it:
//! Push and Pop of the globals (HID 1.11, 6.2.2.7), and Collection and End
//! Collection (6.2.2.4, 6.2.2.6).

use super::error::MapError;
use super::global::Globals;
use super::layout::Layout;
use super::local::Locals;
use alloc::vec::Vec;

/// Fields kept, padding not counted.
pub const FIELDS_MAX: usize = 64;
/// Collections open at once.
pub const DEPTH_MAX: usize = 8;
/// Globals pushed at once.
pub const STACK_MAX: usize = 4;

#[derive(Default)]
pub(super) struct Parser {
    pub g: Globals,
    pub stack: Vec<Globals>,
    pub l: Locals,
    pub depth: usize,
    pub layout: Layout,
    /// Whether the reports carry IDs, once the first one says.
    pub numbered: Option<bool>,
}

impl Parser {
    pub(super) fn push(&mut self) -> Result<(), MapError> {
        if self.stack.len() == STACK_MAX {
            return Err(MapError::TooMany);
        }
        self.stack.push(self.g);
        Ok(())
    }

    pub(super) fn pop(&mut self) -> Result<(), MapError> {
        self.g = self.stack.pop().ok_or(MapError::Shape)?;
        Ok(())
    }

    /// A collection opened, or closed if `open` is false.
    pub(super) fn nest(&mut self, open: bool) -> Result<(), MapError> {
        self.depth = match open {
            true if self.depth < DEPTH_MAX => self.depth + 1,
            true => return Err(MapError::TooMany),
            false => self.depth.checked_sub(1).ok_or(MapError::Shape)?,
        };
        Ok(())
    }
}
