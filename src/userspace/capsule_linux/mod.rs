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

//! The Linux personality capsule: its signed artifacts baked into the
//! kernel, and the spawn that admits them.

mod embed;
mod family;
mod install;
mod role_spawn;
mod roles;
mod spawn;
mod state;
mod terminal;

pub use family::package_arg;

/// Whether a store-started or windowed Linux program still holds the one run
/// role: its endpoint is released when the process ends, so a second open
/// before then is told it is busy instead of being queued and refused unseen.
pub fn run_busy() -> bool {
    crate::services::registry::lookup_service(roles::RUN.name).is_some()
}
pub use install::{spawn_install, spawn_run, spawn_terminal, spawn_uninstall};
pub use spawn::{spawn_linux_capsule, LINUX_CAPS};
pub use state::shared_state;
pub use terminal::{
    admit_terminal_run, end_terminal_runs_of, is_private_run, run_qwen_for_caller,
    terminal_run_gone,
};
