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

//! One method's activation: its arguments, its locals, and where it stands.

use crate::namespace::Id;
use crate::object::Object;
use alloc::rc::Rc;
use core::cell::RefCell;

/// A local or argument, shared so RefOf can point at it.
pub type Slot = Rc<RefCell<Object>>;

pub struct Frame {
    pub table: u16,
    /// The method, or the scope load-time code runs in.
    pub scope: Id,
    pub args: [Slot; 7],
    pub locals: [Slot; 8],
    /// Where the namespace journal stood when the method began, or None
    /// for load-time code, whose definitions are permanent.
    pub journal: Option<usize>,
    /// The serialized method's own mutex, released when it returns.
    pub serial: Option<Id>,
}

fn slot() -> Slot {
    Rc::new(RefCell::new(Object::Uninit))
}

impl Frame {
    pub fn new(table: u16, scope: Id) -> Self {
        let args = core::array::from_fn(|_| slot());
        let locals = core::array::from_fn(|_| slot());
        Frame { table, scope, args, locals, journal: None, serial: None }
    }
}
