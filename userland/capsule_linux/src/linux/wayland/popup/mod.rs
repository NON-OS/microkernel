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
//! xdg_popup: a menu or other transient surface, placed by a positioner
//! against its parent and composed into the parent's window.

pub mod axis;
pub mod ends;
pub mod gone;
pub mod grab;
pub mod layer;
pub mod made;
pub mod place;
pub mod pos;
pub mod positioner;
pub mod rules;
pub mod side;
pub mod stack;
