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

//! The cell source the included link code reaches for.

/*
 * The modules are public rather than private re-exports. A private module whose
 * constants this crate does not happen to call is dead code to clippy, and the
 * honest answer is that the whole command table is part of what these proofs
 * expose, not that the unused half should be hidden or allowed.
 */
#[path = "../../../capsule_net_anon/src/cell/commands.rs"]
pub mod commands;

#[path = "../../../capsule_net_anon/src/cell/geometry.rs"]
pub mod geometry;

#[path = "../../../capsule_net_anon/src/cell/var.rs"]
pub mod var;

pub use commands::CELL_VERSIONS;
pub use var::VarCell;
