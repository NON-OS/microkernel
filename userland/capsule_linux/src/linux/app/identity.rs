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

//! Which program a guest is running, the box it was published with, and the
//! box it runs in, which is narrower after an exec (identity_exec.rs) or
//! when a policy names a destination list this capsule does not hold.

use nonos_linux_policy::{app_id, policy_hash, LinuxAppPolicy, SyscallSet};

use super::reach::{reach_of, Lists, ANYWHERE};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AppIdentity {
    /// The policy bound to the program, as it was proved.
    pub policy: LinuxAppPolicy,
    pub policy_hash: [u8; 32],
    /// What the kernel's seal is told the family is running.
    pub app_id: [u8; 32],
    /// The services the guest is served now (nonos_linux_policy::mask).
    pub mask: u64,
    /// The calls the guest may complete now.
    pub calls: SyscallSet,
    /// The destination lists its streams out are held to (reach.rs).
    pub reach: Lists,
}

impl AppIdentity {
    /// A guest with no program proved into it yet: no service, no call.
    pub const NOTHING: Self = Self {
        policy: nonos_linux_policy::unlisted([0; 32]),
        policy_hash: [0; 32],
        app_id: [0; 32],
        mask: 0,
        calls: SyscallSet::NONE,
        reach: ANYWHERE,
    };

    /// A program started afresh in the box its policy names, holding no list.
    pub fn of(policy: LinuxAppPolicy) -> Self {
        Self::bound(policy, None)
    }

    /// The same, with the list its policy's net_dests_hash names, if this
    /// capsule holds it; without it the box is served no NET_ANON.
    pub fn bound(policy: LinuxAppPolicy, list: Option<&'static [u8]>) -> Self {
        let policy_hash = policy_hash(&policy.encode());
        let (mask, reach) = reach_of(policy.mask, &policy.net_dests_hash, list);
        Self {
            app_id: app_id(&policy.program_blake3, &policy_hash),
            policy,
            policy_hash,
            mask,
            calls: policy.syscall_set,
            reach,
        }
    }

    /// The capsule's own BusyBox: the default box, and an app_id no verifier takes.
    pub fn built_in(program: [u8; 32]) -> Self {
        Self { app_id: [0; 32], ..Self::of(nonos_linux_policy::unlisted(program)) }
    }
}
