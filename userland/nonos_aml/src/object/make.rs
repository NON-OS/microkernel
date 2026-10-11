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

//! Making objects.

use super::Object;
use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::RefCell;

impl Object {
    pub fn string(bytes: &[u8]) -> Self {
        Object::String(Rc::new(RefCell::new(bytes.to_vec())))
    }

    pub fn buffer(bytes: Vec<u8>) -> Self {
        Object::Buffer(Rc::new(RefCell::new(bytes)))
    }

    pub fn package(elements: Vec<Object>) -> Self {
        Object::Package(Rc::new(RefCell::new(elements)))
    }

    /// Bytes this object holds directly, for the allocation budget.
    pub fn size(&self) -> usize {
        match self {
            Object::String(b) | Object::Buffer(b) => b.borrow().len(),
            Object::Package(p) => p.borrow().len() * core::mem::size_of::<Object>(),
            _ => 0,
        }
    }
}
