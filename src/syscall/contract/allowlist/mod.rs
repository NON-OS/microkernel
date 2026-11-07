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
 * The syscalls a process may make, decided once from its token and kept
 * beside it as two bitmaps over the registry's slots. The per-call check is
 * then a bit test (`decide`) instead of a walk of the capability tables.
 */

mod decide;
mod derive;
mod set;

pub use decide::{allowed, decide, depends_on_launcher};
pub(crate) use derive::for_token;
pub use set::{Allowlist, WORDS};
