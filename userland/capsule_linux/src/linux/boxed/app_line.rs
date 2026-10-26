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
//! The `[BOX-APP]` line a started run's report carries before its `[BOX]`
//! line: the program's policy, the 256 bytes its app_id is made from, so the
//! Terminal can ask nonos.prove for the run's receipt (letter agent-02). The
//! personality's own BusyBox has no app_id, and is named by the default policy
//! of program zero, which is what a receipt of it carries.

use alloc::string::String;

use nonos_linux_policy::{unlisted, LinuxAppPolicy};

use super::report::{hex, VERSION};

/// `app_id` and `policy` are the started program's (AppIdentity).
pub fn policy_of(app_id: &[u8; 32], policy: &LinuxAppPolicy) -> [u8; 256] {
    match *app_id == [0; 32] {
        true => unlisted([0; 32]).encode(),
        false => policy.encode(),
    }
}

pub fn app_line(policy: &[u8; 256]) -> String {
    alloc::format!("[BOX-APP] v={VERSION} policy={}\n", hex(policy))
}
