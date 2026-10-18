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

//! One policy with every field distinct, so a field written to the wrong
//! offset cannot pass for the right one.

use crate::mask::{MODELS_RO, WAYLAND};
use crate::{LinuxAppPolicy, SyscallSet};

pub fn policy() -> LinuxAppPolicy {
    LinuxAppPolicy {
        program_blake3: [0x11; 32],
        mask: WAYLAND | MODELS_RO,
        listing_id: [0x22; 32],
        release_id: [0x33; 32],
        net_dests_hash: [0x44; 32],
        syscall_set: SyscallSet::NONE.with(0).with(1).with(231).with(511).with(9999),
    }
}
