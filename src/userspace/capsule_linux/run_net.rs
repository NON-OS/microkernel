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

//! The run role with the network. init cannot read a policy's mask, so the
//! run role starts without Network; a run whose program's policy has
//! NET_ANON asks, by MkLinuxRunNet, to be started again under this role,
//! which holds Network, binds the same policy again, and exits unless it
//! has NET_ANON. Its guest reaches out only through the route the
//! person chose, never Direct.

use alloc::string::String;
use alloc::vec;

use super::role_spawn::spawn_role;
use super::roles::Role;
use crate::capabilities::Capability;
use crate::kernel_core::process_spawn::capsule_spawn::SpawnError;

pub(super) const RUN_NET: Role = Role {
    name: "app.linux.run.net",
    port: 5204,
    inbox: "endpoint.app.linux.run.net.reply",
    reply_port: 5205,
    tag: b"[LINUX-RUN-NET] elf error:",
    extra_caps: Capability::Network.bit(),
};

/// The run of `package` again, as app.linux.run.net, with the argv init
/// gives every run role.
pub fn spawn_run_net(package: &str) -> Result<u32, SpawnError> {
    let pid = spawn_role(&RUN_NET)?;
    let argv = vec![String::from("run"), String::from(package)];
    let _ = crate::process::set_argv(pid, argv);
    Ok(pid)
}
