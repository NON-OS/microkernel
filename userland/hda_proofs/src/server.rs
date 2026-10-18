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

//! The serving half of the capsule, from the shipping files: the request
//! handlers and the reply they send, and the parts of the serving loop that
//! run between requests (`runner`). The loop itself waits on IPC for ever,
//! so it is not mounted; its tests drive the handlers as it does.

#[path = "../../capsule_driver_hda/src/server/error.rs"]
mod error;
#[path = "../../capsule_driver_hda/src/server/handlers/mod.rs"]
pub mod handlers;
pub mod runner;
