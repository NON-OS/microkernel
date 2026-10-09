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
 * Extended attributes, kept by the family as tmpfs keeps them.
 *
 * The store holds a file's bytes and nothing beside them, so the family's
 * files keep their attributes here, for the family's life, which is the
 * life of its private directories. The shared tree is read-only: its files
 * have none and can be given none (EROFS). /proc and /dev files do not
 * support them, as procfs does not (EOPNOTSUPP).
 */

mod edit;
mod set;
mod table;

pub use edit::{forget, list, remove, renamed};
pub use set::set;
pub use table::{check_name, get};
