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

//! The Etna design system in the wallet capsule: tokens, faces and type
//! roles first, then the parts and the one frame every screen sits in.

pub mod backdrop;
mod band;
pub mod face;
pub mod frame;
pub mod frame_spec;
pub mod groups;
pub mod parts;
pub mod rect;
mod roles;
pub mod symbol;
pub mod text;
pub mod tokens;
pub mod wrap;

pub use roles::Role;
