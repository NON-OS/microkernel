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

//! The box for a program that arrived with no policy of its own.
//!
//! The owner's decision (O1). What it holds is a terminal, its own home and
//! exec, which keeps command-line tools working while nothing reaches the
//! display or the network without a policy that says so. Changing the
//! decision is changing UNLISTED_MASK.

use crate::mask::{EXEC, FILES_HOME, TTY};
use crate::policy::LinuxAppPolicy;
use crate::syscall_set::SyscallSet;

pub const UNLISTED_MASK: u64 = TTY | FILES_HOME | EXEC;

/// The policy capsule_linux applies to `program_blake3` when none is bound
/// to it. No listing, no release, and every call may complete: the box is
/// the mask alone.
pub const fn unlisted(program_blake3: [u8; 32]) -> LinuxAppPolicy {
    LinuxAppPolicy {
        program_blake3,
        mask: UNLISTED_MASK,
        listing_id: [0; 32],
        release_id: [0; 32],
        net_dests_hash: [0; 32],
        syscall_set: SyscallSet::ALL,
    }
}
