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

//! DOM host callbacks the QuickJS binding layer calls into. The engine carries
//! a raw pointer to the page DOM as its context opaque; these functions read and
//! mutate that tree. Script execution is synchronous, so the DOM is not touched
//! elsewhere while a callback runs. Strings returned to C are malloc'd with the
//! engine allocator and freed by the C side.

mod attrs;
mod create;
mod ffi;
mod graft;
mod lookup;
mod nav;
mod siblings;
mod style;
mod text;

pub(crate) use ffi::{cdup, cstr, dom};
