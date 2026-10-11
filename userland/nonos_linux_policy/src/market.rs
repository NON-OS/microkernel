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

//! Whether a policy can back an isolation proof. capsule_linux
//! serves exit and exit_group whatever the set says, so a set without them
//! would let them succeed outside it. A guest's native call is a seal row in
//! bucket 512, so a set with 512 would count one that went through as
//! declared. An all-ones set is therefore never market ready. Nor is a box
//! with FILES_USER: nothing serves the person's files yet, and a bit that
//! grants nothing today would grant them silently the day something did
//!.

use crate::mask::FILES_USER;
use crate::policy::LinuxAppPolicy;

/// x86_64 exit and exit_group, the calls capsule_linux always serves.
pub const ALWAYS_SERVED: [u64; 2] = [60, 231];
/// The bucket every native call and every number past 511 lands in.
pub const NATIVE_BUCKET: u64 = 512;

/// The rule the market's install_ready and the pack tool hold a release to.
pub fn market_ready(policy: &LinuxAppPolicy) -> bool {
    let set = &policy.syscall_set;
    ALWAYS_SERVED.iter().all(|&nr| set.allows(nr))
        && !set.allows(NATIVE_BUCKET)
        && policy.mask & FILES_USER == 0
}
