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

//! The syscalls any valid token may make, and attestation, the device
//! secret and the install image (mod.rs chains the rest).

use crate::capabilities::CapabilityToken;
use crate::syscall::numbers::SyscallNumber;

pub(super) fn check(caps: &CapabilityToken, number: SyscallNumber) -> Option<bool> {
    Some(match number {
        SyscallNumber::MkExit
        | SyscallNumber::MkPidAlive
        | SyscallNumber::MkYield
        | SyscallNumber::MkFutexWait
        | SyscallNumber::MkFutexWake
        | SyscallNumber::MkTimeMillis
        | SyscallNumber::MkTimeMonotonic
        | SyscallNumber::MkTimeRtc
        | SyscallNumber::MkBatteryStatus
        | SyscallNumber::MkProcStat
        | SyscallNumber::MkAttestStatus
        | SyscallNumber::MkAttestPolicy
        | SyscallNumber::MkBootAttest
        | SyscallNumber::MkCapCheck => caps.is_valid(),

        /*
         * The document is signed by the TPM's attestation key, which derives
         * from the endorsement seed under a fixed template: the same key on
         * every boot and across a reinstall. Whoever holds a quote holds a
         * hardware identifier that outlives everything else this machine
         * forgets. So it goes only to a capsule that reads the registry, and
         * never to one that can reach the network itself; the quote leaves
         * the machine only when the user carries it out.
         */
        SyscallNumber::MkAttestDoc => caps.can_attest_doc(),

        /*
         * The entries name every running capsule with its measurement and
         * capability mask, the machine's inventory, so reading them takes a
         * capability of its own, which the programs that render a receipt hold.
         */
        SyscallNumber::MkAttestEntries => caps.can_attest_read(),
        SyscallNumber::MkLogTail => caps.can_attest_read(),
        /* The witness of the anonymous device proof, for nonos.prove alone. */
        SyscallNumber::MkDeviceSecret => caps.can_device_secret(),
        /* Its boot slots, which narrow down the release: for the prover alone too. */
        SyscallNumber::MkBootSlots => caps.can_device_secret(),
        /* The EK and the AK name this machine for its TPM's life: the prover's. */
        SyscallNumber::MkEnroll => caps.can_device_secret(),
        /*
         * The bytes of the image this machine booted, for writing to a
         * disk. Public bytes, but a hundred megabytes of them; the same
         * authority that lists disks is the one that installs to them.
         */
        SyscallNumber::MkInstallSource => caps.can_install_source(),

        _ => return None,
    })
}
