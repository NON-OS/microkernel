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

//! The shipped `Regs`, every access made through it, with a test's part able
//! to answer one register's reads (`on_read`): count them, or hold a bit on.

use std::cell::RefCell;

#[path = "../../capsule_driver_e1000/src/regs.rs"]
mod shipped;

type Answer = Box<dyn Fn(u32) -> u32>;

thread_local! {
    static ANSWER: RefCell<Option<(usize, Answer)>> = const { RefCell::new(None) };
}

/// The part answering this thread's reads of `offset` until it is dropped.
pub struct OnRead;

impl Drop for OnRead {
    fn drop(&mut self) {
        ANSWER.with(|a| a.borrow_mut().take());
    }
}

/// Reads of `offset` on this thread return `f` of what the window holds.
pub fn on_read(offset: usize, f: impl Fn(u32) -> u32 + 'static) -> OnRead {
    ANSWER.with(|a| *a.borrow_mut() = Some((offset, Box::new(f))));
    OnRead
}

fn answer(offset: usize, held: u32) -> u32 {
    let mine = |(at, _): &&(usize, Answer)| *at == offset;
    ANSWER.with(|a| a.borrow().as_ref().filter(mine).map_or(held, |(_, f)| f(held)))
}

#[derive(Clone, Copy)]
pub struct Regs(shipped::Regs);

impl Regs {
    pub const fn new(base: u64) -> Self {
        Self(shipped::Regs::new(base))
    }

    /// # Safety
    /// As the shipped `r32`.
    pub unsafe fn r32(&self, offset: usize) -> u32 {
        answer(offset, self.0.r32(offset))
    }

    /// # Safety
    /// As the shipped `w32`.
    pub unsafe fn w32(&self, offset: usize, value: u32) {
        self.0.w32(offset, value)
    }
}
