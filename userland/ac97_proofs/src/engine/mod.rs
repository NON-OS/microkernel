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

//! The driver's engine files by path, the descriptor rule public.

#[path = "../../../capsule_driver_ac97/src/engine/bdl.rs"]
pub mod bdl;
#[path = "../../../capsule_driver_ac97/src/engine/control.rs"]
mod control;
#[path = "../../../capsule_driver_ac97/src/engine/position.rs"]
mod position;

pub use bdl::{entry, fill};
pub use control::{start, stop};
pub use position::follow;
