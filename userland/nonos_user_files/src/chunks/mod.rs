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

//! A file's bytes: its chunks on the volume, and the ones changed since,
//! held in RAM up to a fixed count across every file.
//!
//! What a file holds past its size is never kept: a write keeps a chunk's
//! bytes up to the write's end, and a shrinking truncate rewrites the chunk
//! it ends in, so a chunk or object never holds bytes a later grow would
//! show in place of zeros.

mod dirty;
mod read;
mod size;
mod write;

pub use dirty::Dirty;
pub use read::read;
pub use size::truncate;
pub use write::write;

use crate::host::Objects;
use crate::store::Store;
use crate::tree::Tree;

/// What a chunk operation works on, borrowed from the broker together.
pub struct Files<'a, O: Objects> {
    pub objects: &'a mut O,
    pub tree: &'a mut Tree,
    pub store: &'a mut Store,
    pub dirty: &'a mut Dirty,
}
