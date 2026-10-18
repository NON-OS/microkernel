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

//! An object.

use crate::namespace::Id;
use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::RefCell;

/// A buffer's or a string's bytes, shared so that a reference made by Index
/// writes into the object it was taken from. A string is kept without its NUL.
pub type Buf = Rc<RefCell<Vec<u8>>>;
/// A package's elements, shared for the same reason.
pub type Pkg = Rc<RefCell<Vec<Object>>>;

#[derive(Clone, Debug)]
pub enum Object {
    /// A local or argument never stored to, or a method that returned nothing.
    Uninit,
    Integer(u64),
    String(Buf),
    Buffer(Buf),
    Package(Pkg),
    Ref(Ref),
    /// A named object that holds no data of its own (a Device, a Mutex, a
    /// Method passed as an argument), with the generation of its slot.
    Named(Id, u32),
    /// A DDBHandle: the index of a table Load or LoadTable added.
    Table(u16),
}

/// What RefOf, Index and CondRefOf return (section 19.6.115 and 19.6.62).
#[derive(Clone, Debug)]
pub enum Ref {
    /// A named object, with the generation of its slot.
    Named(Id, u32),
    /// A local or an argument.
    Slot(Rc<RefCell<Object>>),
    /// One element of a package.
    Element(Pkg, usize),
    /// One byte of a buffer or string.
    Byte(Buf, usize),
}
