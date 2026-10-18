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

//! The keyboard files, with `active` replaced: see active.rs.

mod active;
#[path = "../../../capsule_driver_virtio_input/src/keyboard/codes/mod.rs"]
pub mod codes;
#[path = "../../../capsule_driver_virtio_input/src/keyboard/modifiers.rs"]
pub mod modifiers;
#[path = "../../../capsule_driver_virtio_input/src/keyboard/publish.rs"]
mod publish;
#[path = "../../../capsule_driver_virtio_input/src/keyboard/state.rs"]
mod state;

pub use active::{cycles, set_layout};
pub use state::Keyboard;
