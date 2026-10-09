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

//! The Extended CSD (JEDEC eMMC 5.1, 7.4): 512 bytes read with CMD8.

mod bus_test;
mod hs;
mod offsets;
mod register;
mod sizes;
mod switch_time;
mod values;

pub use offsets::*;
pub use register::*;
pub use switch_time::*;
pub use values::*;
