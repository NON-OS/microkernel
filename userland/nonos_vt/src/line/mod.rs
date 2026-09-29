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

//! One row of cells, on screen or in scrollback.
//!
//! A screen line is always as wide as the screen. A scrollback line may be
//! shorter: trailing blanks are dropped when it leaves the screen, and a
//! reader asking past its end gets a blank. Every write keeps two-column
//! characters whole: overwriting either half blanks the other.

mod data;
mod marks;
mod shift;
mod trim;
mod write;

pub use data::Line;
