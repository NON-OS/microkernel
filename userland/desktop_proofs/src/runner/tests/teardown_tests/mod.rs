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

//! A closed window taken down when one of the calls fails
//! (app_skeleton runner/teardown_steps.rs). The window manager hears of the
//! close whatever the compositor answered: before, a scene remove answered
//! late kept the window in the window manager's table, a target for presses
//! with nothing drawn there, and the dock kept the app marked running.

mod log;
mod steps;
