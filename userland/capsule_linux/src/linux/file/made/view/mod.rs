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

/*
 * The family as /proc shows it to the guest being answered.
 *
 * Only the family's own processes are here, each under the number the
 * guest's pid namespace gives it; nothing outside the family is, so no
 * path under /proc can name it. The serve loop fills this before a call
 * that may read /proc and empties it after, so it is never stale.
 */

mod lent;
mod proc;
mod shape;

pub use lent::{lend, with};
pub use proc::Proc;
pub use shape::{Open, View};
