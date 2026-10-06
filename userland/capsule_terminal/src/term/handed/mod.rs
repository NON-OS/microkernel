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

//! A command the desktop shell hands this Terminal: a Launchpad tool tile
//! runs its tool here, or leaves it on the prompt for its arguments. The shell
//! holds it the way it holds a path for the editor, and the Terminal asks for
//! it on its ticks (terminal::take_handed).

mod ask;
pub mod cadence;
pub mod parse;
pub mod pick;

pub use ask::ask;
pub use parse::{parse, Handed};
pub use pick::{pick, Active, Pick};
