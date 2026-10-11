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

//! Copies that share nothing with their source.

use super::Object;
use alloc::vec::Vec;

/// A package can be made to hold itself at run time, through stores into its
/// own elements; a copy that deep is refused rather than overflowing the
/// capsule's stack.
const MAX_COPY_DEPTH: u32 = 32;

impl Object {
    /// What Store and a method call hand on: strings, buffers and packages are
    /// copied, element by element, so writing to the copy leaves the source
    /// as it was. References and named objects are themselves the value.
    pub fn deep_copy(&self) -> Option<Object> {
        self.copy_at(0)
    }

    fn copy_at(&self, depth: u32) -> Option<Object> {
        if depth > MAX_COPY_DEPTH {
            return None;
        }
        Some(match self {
            Object::String(s) => Object::string(&s.borrow()),
            Object::Buffer(b) => Object::buffer(b.borrow().clone()),
            Object::Package(p) => {
                let elems = p.borrow();
                let copied: Option<Vec<Object>> =
                    elems.iter().map(|e| e.copy_at(depth + 1)).collect();
                Object::package(copied?)
            }
            other => other.clone(),
        })
    }
}
