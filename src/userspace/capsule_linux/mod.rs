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
mod role_term_net;
mod roles;
mod run_net;
mod run_owner;
mod spawn;
mod state;
mod term_ticket;
mod term_ticket_rule;
mod terminal;

pub use family::package_arg;
pub use install::{spawn_install, spawn_run, spawn_terminal, spawn_terminal_net, spawn_uninstall};
pub use run_net::spawn_run_net;
/// What the terminal's network role asks beyond LINUX_CAPS: the tool bound
/// checks the caller against exactly what that role's spawn installs.
pub const TERMINAL_NET_EXTRA: u64 = role_term_net::TERMINAL_NET.extra_caps;
pub use run_owner::{run_busy, run_net_busy, run_package_of};
pub(crate) use embed::LINUX_MANIFEST_BYTES;
pub use spawn::{spawn_linux_capsule, LINUX_CAPS};
pub use state::shared_state;
pub use term_ticket::{asked_by_terminal_role, end_tickets_of, take_term_ticket};
pub use terminal::{
    admit_terminal_run, end_terminal_runs_of, is_private_run, run_qwen_for_caller,
    terminal_run_gone,
};
