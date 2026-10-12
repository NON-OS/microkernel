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

//! One model reached two ways, as the capsule reaches one engine: its
//! registers through `Mmio` (BAR0) and its memory through `Frames` (the
//! scanout surfaces' grants).

use std::cell::RefCell;
use std::rc::Rc;

use super::gen9::Gen9;
use crate::engine::access::Mmio;

#[derive(Clone)]
pub struct Shared(pub Rc<RefCell<Gen9>>);

impl Mmio for Shared {
    fn read(&mut self, offset: u32) -> u32 {
        self.0.borrow_mut().read(offset)
    }
    fn write(&mut self, offset: u32, value: u32) {
        self.0.borrow_mut().write(offset, value)
    }
    fn write64(&mut self, offset: u32, value: u64) {
        self.0.borrow_mut().write64(offset, value)
    }
    fn read64(&mut self, offset: u32) -> u64 {
        self.0.borrow_mut().read64(offset)
    }
}
