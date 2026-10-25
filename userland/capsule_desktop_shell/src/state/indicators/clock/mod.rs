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

//! The menubar's date and time line, from a moment already read: "Sat 03 Oct
//! 19:42" on a 24 hour clock, "Sat 03 Oct  7:42 PM" on a 12 hour one.
//!
//! The 12 hour clock used to drop the 24 hour hour into twelve and show no
//! AM or PM, so 07:42 in the morning and the evening read the same.

mod calendar;
mod line;

pub use line::{line, Moment, LINE_MAX};
