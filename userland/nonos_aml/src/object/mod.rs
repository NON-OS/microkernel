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

//! The values AML computes with (section 19.3.5) and the conversions between
//! them.

mod compare;
mod convert;
mod convert_text;
mod copy;
mod explicit;
mod explicit_buffer;
mod explicit_text;
mod hex;
mod kinds;
mod make;
mod value;
mod width;

pub use compare::compare;
pub use explicit::{from_bcd, to_bcd};
pub use explicit_buffer::buffer_to_string;
pub use kinds::{type_of, ObjectType};
pub use value::{Buf, Object, Pkg, Ref};
pub use width::Width;
