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
//! The GGTT confined (G-75): every rig that builds a driver on frames arms
//! the model's watch with them, so every entry written from then on is
//! held to naming a page the driver holds (`model/confine.rs`).

mod heads_tests;
pub mod kit;
mod life_tests;
mod self_tests;

pub use kit::arm;
