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
//! The tiling layout: workspaces of windows in split trees, with floating
//! windows beside them (letter gpu 09, step 1). Pure; held in wm_proofs.

pub mod desk;
mod desk_send;
pub mod insert;
pub mod neighbour;
pub mod node;
pub mod place;
pub mod remove;
pub mod types;
pub mod workspace;
mod workspace_float;
