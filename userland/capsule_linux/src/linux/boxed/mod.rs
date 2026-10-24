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

//! `linux --box <preset> <program>`: a program run in a named box, and the
//! report of everything the box refused it.

mod allowed;
mod dests;
mod finish;
mod folders;
mod hash;
mod items;
mod kind;
mod line;
mod preset;
mod report;
mod state;
mod tally;

pub use finish::finish;
pub use kind::{of_needs, Kind};
pub use preset::parse;
pub use state::{arm, call_begins, may_write, narrow, note};
