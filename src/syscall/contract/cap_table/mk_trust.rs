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

//! Local signing, seals, installing apps and prove jobs.

use crate::capabilities::CapabilityToken;
use crate::syscall::numbers::SyscallNumber;

pub(super) fn check(caps: &CapabilityToken, number: SyscallNumber) -> Option<bool> {
    Some(match number {
        /*
         * Minting a trailer is a trust operation, so it sits with the
         * enrolment calls rather than with the installer ones.
         */
        SyscallNumber::MkLocalSign => caps.can_local_sign(),

        /* Asking is not minting. */
        SyscallNumber::MkLocalVerify => caps.can_foreign_exec(),

        /*
         * A family's seal is its supervisor's to note in. Reading one hands out
         * the private witness, so it is nonos.prove's alone; the handler also
         * asks for Vendor authority.
         */
        SyscallNumber::MkSealNote => caps.can_foreign_exec(),
        SyscallNumber::MkSealRead => caps.can_device_secret(),
        /* A receipt's peek and its one drop, by the same gate (seal/sys_keep.rs). */
        SyscallNumber::MkSealPeek | SyscallNumber::MkSealDrop => caps.can_device_secret(),

        /* The right to ask, which is not the right to host. */
        SyscallNumber::MkAppInstall
        | SyscallNumber::MkAppLaunch
        | SyscallNumber::MkAppInstallStatus
        | SyscallNumber::MkAppUninstall => caps.can_app_install(),
        /* Who may ask is the handler's to say, by kind. */
        SyscallNumber::MkProveJob => caps.can_ipc(),

        _ => return None,
    })
}
