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

//! Which program a guest is running, the box it was published with, and
//! the box it actually runs in.
//!
//! The two boxes differ after an exec: the new program brings its own
//! policy, but runs with only what both it and the program that exec'd it
//! were given. So a shell with no network cannot exec its way onto one.

use nonos_linux_policy::{app_id, policy_hash, LinuxAppPolicy, SyscallSet};

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
}

impl AppIdentity {
    /// A guest no program has been proved into yet: served nothing, and no
    /// call it makes may complete.
    pub const NOTHING: Self = Self {
        policy: nonos_linux_policy::unlisted([0; 32]),
        policy_hash: [0; 32],
        app_id: [0; 32],
        mask: 0,
        calls: SyscallSet::NONE,
    };

    /// A program started afresh, in exactly the box its policy names.
    pub fn of(policy: LinuxAppPolicy) -> Self {
        let policy_hash = policy_hash(&policy.encode());
        Self {
            app_id: app_id(&policy.program_blake3, &policy_hash),
            policy,
            policy_hash,
            mask: policy.mask,
            calls: policy.syscall_set,
        }
    }

    /// `program`, exec'd by the guest this is: its identity, inside what
    /// both were allowed.
    pub fn exec(&self, program: &AppIdentity) -> Self {
        Self {
            mask: nonos_linux_policy::mask::narrow(self.mask, program.mask),
            calls: self.calls.meet(&program.calls),
            ..*program
        }
    }
}
