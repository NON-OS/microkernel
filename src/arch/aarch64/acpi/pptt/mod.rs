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

//! PPTT (ACPI 6.5, 5.2.30): which processors share a package, a cluster or
//! a core, and which caches each level holds.

mod cache;
mod check;
mod looping;
mod structs;
mod walk;

pub use cache::Cache;
pub use structs::ProcessorNode;
pub use walk::{pptt, Pptt};
