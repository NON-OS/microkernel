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

// The terminal's modules, included at the crate root by lib.rs.

#[allow(clippy::all)]
#[path = "../../capsule_terminal/src/command/mod.rs"]
mod command;
#[allow(clippy::all)]
#[path = "../../capsule_terminal/src/event/mod.rs"]
mod event;
#[allow(clippy::all)]
#[path = "../../capsule_terminal/src/git/mod.rs"]
mod git;
#[allow(clippy::all)]
#[path = "../../capsule_terminal/src/jobs/mod.rs"]
mod jobs;
#[allow(clippy::all)]
#[path = "../../capsule_terminal/src/layout/mod.rs"]
mod layout;
#[allow(clippy::all)]
#[path = "../../capsule_terminal/src/mixnet/mod.rs"]
mod mixnet;
#[allow(clippy::all)]
#[path = "../../capsule_terminal/src/paint/mod.rs"]
mod paint;
#[allow(clippy::all)]
#[path = "../../capsule_terminal/src/palette/mod.rs"]
mod palette;
#[allow(clippy::all)]
#[path = "../../capsule_terminal/src/rail/mod.rs"]
mod rail;
#[allow(clippy::all)]
#[path = "../../capsule_terminal/src/term/mod.rs"]
pub mod term;
