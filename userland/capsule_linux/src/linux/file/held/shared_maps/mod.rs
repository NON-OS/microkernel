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

//! A stored file mapped MAP_SHARED, served within the family.
//!
//! On Linux such a map and the file are the same pages. No peer call gives
//! two address spaces one frame, so here each map is its own pages, and the
//! family keeps a table of them. Every write of the file's bytes is copied
//! into each map that shows them (echo.rs); a cut or a growth zeroes what
//! lies between the two ends. Every read of the file, and every flush of
//! the family's copy, first takes back the pages a writer changed
//! (sync.rs, back.rs). Both are sound because a range of a file has at
//! most one map that may write it (rules.rs), so a writer's pages have no
//! other map to reach. A fork's copy of a map is private and is not in the
//! table.

mod asks;
mod back;
mod echo;
mod ends;
mod entry;
mod follow;
mod life;
mod names;
mod protect;
mod rules;
mod spans;
mod sync;
mod table;

pub use echo::{truncated, wrote};
pub use ends::{ended, holds, unmapped};
pub use follow::{renamed, unlinked};
pub use life::{admit, asked, record};
pub use protect::protect;
pub use sync::{before_flush, before_flush_all, before_read, synced};
