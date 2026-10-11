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

//! The fixed bounds every walk of a blob works within.

/// Largest blob accepted. Linux stops at 2 MiB as well; a header claiming more
/// is far likelier corrupt than a real board.
pub const MAX_BLOB: usize = 2 << 20;

/// Deepest node nesting accepted, the root at depth 0. Real trees stop around
/// six levels; the bound is what lets every per-depth table be fixed storage.
pub const MAX_DEPTH: usize = 16;
