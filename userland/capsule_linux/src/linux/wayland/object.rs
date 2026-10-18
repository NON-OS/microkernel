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

//! The client's object table.

#[path = "object_kind.rs"]
mod kind;
#[path = "object_life.rs"]
mod life;
#[path = "object_table.rs"]
mod table;
#[path = "object_version.rs"]
mod version;

pub use kind::Object;
pub use table::Objects;

/// The most objects one client holds at once. A real client holds a few
/// hundred; without a ceiling, a client creating objects it never destroys
/// would grow this capsule's memory, and slow every lookup, for good.
pub const MAX_OBJECTS: usize = 4096;
