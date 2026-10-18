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

//! Opening an app again (state/taskbar/route.rs, expect.rs). A dock click on
//! an app marked running raises a window it has, forgets a window whose
//! process is gone, and opens the app afresh once none is left; a launch
//! whose window never comes is said.

mod alive;
mod choice;
mod marks;
mod rig;
mod wait;
