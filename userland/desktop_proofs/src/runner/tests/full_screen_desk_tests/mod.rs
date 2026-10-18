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

//! The green button end to end: app_skeleton's full-screen rect, the window
//! manager's maximise, the compositor's scene, and the hit test, for every
//! app (they all go through maximize::toggle), at every display scale.

mod ask;
mod dock_back;
mod dock_edge;
mod dock_hides;
mod dock_rig;
mod green;
mod rig;
