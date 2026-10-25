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

//! Which app a registered service name belongs to. The dock starts extra
//! windows of an app as numbered instances: the kernel registers the first
//! window under the app's own name ("app.terminal") and each further one
//! under the name its signed spawn table declares, the base name, a dot and a
//! decimal slot ("app.terminal.1" to "app.terminal.3"). Every place the shell
//! asks which app a window or a request came from goes through this one
//! matcher, so an instance counts as its app and nothing else does.

pub mod matcher;
mod name;

pub use matcher::{app_index_of, INSTANCE_SLOTS};
pub use name::{instance_name, INSTANCE_NAME_MAX};
