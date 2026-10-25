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

//! What the display server gives the rest of the personality, named at `crate::linux::wayland` by mod.rs.

pub use super::close::close_window::{close_window, disconnect};
pub use super::input::pump::pump;
pub use super::object::Objects;
pub use super::scene::Scene;
pub use super::serve::serve;
pub use super::window_life::ends_connection;
