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

//! Every STARK proof this boot checked, one row each, read off what the
//! kernel reports: the kernel the bootloader admitted, the bootloader the
//! kernel checked, and the capsules the kernel checked at spawn.

mod capsules;
mod kernel;
mod loader;
mod mark;
mod policy;
mod row;

pub use mark::Mark;
pub use row::{rows, Row};
