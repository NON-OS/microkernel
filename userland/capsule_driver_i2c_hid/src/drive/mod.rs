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

//! Driving the pad between two waits of the serving loop: when to look for
//! it and read it, its interrupt line, and a reset it did on its own. The
//! loop around these is server/runner/run.rs.

mod doorbell;
mod mouse_run;
mod pace;
mod pass;
mod rang;
mod recover;
mod reset;

pub use doorbell::Doorbell;
pub use pace::Pace;
pub use pass::pass;
pub use recover::Recover;
pub use reset::{configure, mouse_seen};
