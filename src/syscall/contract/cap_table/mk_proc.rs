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

//! Trust roots, the clock, memory and capsule loading.

use crate::capabilities::{Capability, CapabilityToken};
use crate::syscall::numbers::SyscallNumber;

pub(super) fn check(caps: &CapabilityToken, number: SyscallNumber) -> Option<bool> {
    Some(match number {
        /*
         * Enrolling a signing root changes which software this machine will
         * start, so it is gated on its own capability rather than on Admin. The
         * handler asks the live token again and a human confirms out of band;
         * this is the first of the three refusals, not the only one.
         */
        SyscallNumber::MkDevRootRequest
        | SyscallNumber::MkDevRootConfirm
        | SyscallNumber::MkDevRootLocal
        | SyscallNumber::MkLocalConsent
        | SyscallNumber::MkLocalRestore => caps.can_enrol_dev_root(),

        SyscallNumber::MkTimeAdjust => caps.can_set_time(),

        SyscallNumber::MkMmap => caps.can_allocate_memory(),
        SyscallNumber::MkMunmap => caps.can_deallocate_memory(),

        SyscallNumber::MkCapsuleLoad => {
            caps.is_valid()
                && caps.grants_all(&[Capability::CoreExec, Capability::IPC, Capability::Memory])
        }

        SyscallNumber::MkCapsuleVerify => {
            caps.is_valid()
                && caps.grants_all(&[Capability::CoreExec, Capability::IPC, Capability::Memory])
        }

        _ => return None,
    })
}
