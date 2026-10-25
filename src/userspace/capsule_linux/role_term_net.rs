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

//! The terminal's network role, beside the others (roles.rs).

use super::roles::Role;
use crate::capabilities::Capability;

/// The terminal's run of a `--box` naming destinations: the one terminal
/// role with Network, spawned only for such a box (tool_capsules/net_box.rs).
/// The personality refuses to run in it without a destination list, and a
/// guest's streams go only to that list, over the route the person chose.
///
/// The tool bound (tool_capsules/bound.rs) refuses this role to a caller
/// without Network, but the Terminal passes it as a launcher (SpawnWindow).
/// For a Terminal user the gate is the box rule: the kernel's argv check,
/// then the personality's (boxed/terminal_run.rs), NET_ANON only for a
/// program with no policy of its own, narrowed by the box (app/identity_box.rs).
pub(super) const TERMINAL_NET: Role = Role {
    name: "app.linux.term.net",
    port: 5188,
    inbox: "endpoint.app.linux.term.net.reply",
    reply_port: 5189,
    tag: b"[LINUX-TERM-NET] elf error:",
    extra_caps: Capability::Network.bit(),
};
