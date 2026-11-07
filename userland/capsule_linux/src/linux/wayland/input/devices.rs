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

//! Every wl_pointer or wl_keyboard the client holds. A seat's input goes to
//! each of them, as a compositor sends it: a toolkit and the program over it
//! may each make their own from a seat of their own (Firefox's GDK and its
//! widget layer do), and the one not told sees no input at all.

use alloc::vec::Vec;

use crate::linux::guest::Guest;

use super::super::object::Object;

pub fn pointers(guest: &Guest) -> Vec<u32> {
    guest.objects.all(Object::Pointer).collect()
}

pub fn keyboards(guest: &Guest) -> Vec<u32> {
    guest.objects.all(Object::Keyboard).collect()
}
