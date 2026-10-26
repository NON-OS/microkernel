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

//! The personality, spawned to install a package or to run one, rather than
//! to host the built-in program.

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use super::role_spawn::spawn_role;
use super::role_term_net::TERMINAL_NET;
use super::roles::{Role, INSTALL, RUN, TERMINAL};
use crate::kernel_core::process_spawn::capsule_spawn::SpawnError;

/// Spawn the installer for `package`, which must hash to `pinned`. `market`
/// is the release's guest_policy | listing_key | release_key, and None only
/// for a release that has no guest_policy (plan/30): sent as 640 hex, or "-".
/// With `direct`, a Qwen tier's model is downloaded over a direct connection
/// for this install only: the person chose it in the store, where it is
/// said that the mirror then sees this machine's address.
pub fn spawn_install(
    package: &str,
    pinned: &[u8; 32],
    market: Option<&[u8; 320]>,
    direct: bool,
) -> Result<u32, SpawnError> {
    let m = market.map_or_else(|| String::from("-"), |m| hex(m));
    let mut argv = vec![String::from("install"), String::from(package), hex(pinned), m];
    if direct {
        argv.push(String::from("direct"));
    }
    spawn(&INSTALL, argv)
}

/// Spawn the personality to take away what installing `package` put down.
/// It runs as the installer does, so one install or removal runs at a time.
pub fn spawn_uninstall(package: &str) -> Result<u32, SpawnError> {
    spawn(&INSTALL, vec![String::from("uninstall"), String::from(package)])
}

/// Spawn the personality to run the program `package` installed.
pub fn spawn_run(package: &str) -> Result<u32, SpawnError> {
    spawn(&RUN, vec![String::from("run"), String::from(package)])
}

/// Spawn the personality for the terminal's `linux` command. `argv` is the
/// command as typed: `linux`, the program, then its arguments.
pub fn spawn_terminal(argv: Vec<String>) -> Result<u32, SpawnError> {
    spawn(&TERMINAL, argv)
}

/// The same, in the terminal role that holds Network (roles::TERMINAL_NET).
pub fn spawn_terminal_net(argv: Vec<String>) -> Result<u32, SpawnError> {
    spawn(&TERMINAL_NET, argv)
}

fn spawn(role: &Role, argv: Vec<String>) -> Result<u32, SpawnError> {
    let pid = spawn_role(role)?;
    let _ = crate::process::set_argv(pid, argv);
    Ok(pid)
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| alloc::format!("{b:02x}")).collect()
}
