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

//! The claims themselves.

use super::super::live::{CAP_ADMIN, CAP_DEBUG};
use super::probe::Probe;

pub struct Invariant {
    pub name: &'static [u8],
    pub claim: &'static [u8],
    pub mechanism: &'static [u8],
    pub probe: Probe,
}

pub const INVARIANTS: &[Invariant] = &[
    Invariant {
        name: b"NO LOGS",
        claim: b"no shipped capsule may emit MkDebug or open a serial surface",
        mechanism: b"every shipped Capsule.mk has Debug bit absent from CAPSULE_REQUIRED_CAPS; kernel rejects MkDebug syscalls outside the mask",
        probe: Probe::NoneHold(CAP_DEBUG),
    },
    Invariant {
        name: b"NO TRACES",
        claim: b"no persistent user identifier or content survives a capsule exit",
        mechanism: b"every shipped capsule refuses FileSystem cap unless explicitly granted; clipboard has idle auto-clear; input_router holds no history",
        probe: Probe::NotAtRuntime,
    },
    Invariant {
        name: b"EPHEMERAL",
        claim: b"all state is RAM-resident; no on-disk record exists unless a capsule declares FileSystem in its mask",
        mechanism: b"only ramfs + vfs touch disk surfaces; the trust keystore is read-only at boot",
        probe: Probe::NotAtRuntime,
    },
    Invariant {
        name: b"NOT LINUX",
        claim: b"no POSIX shapes, no errno tables, no fd numbering, no signal model",
        mechanism: b"Mk* 4-byte ASCII tag syscall ABI; NCMP-style wire across every capsule; capability taxonomy is NONOS-native",
        probe: Probe::NotAtRuntime,
    },
    Invariant {
        name: b"PRIVACY MICROKERNEL",
        claim: b"every capsule runs CPL=3 with a static capability mask the kernel enforces at every syscall",
        mechanism: b"capsule_spawn::spawn_verified records caps_bits; syscall dispatch checks cap mask before every routed handler; mask is signed in the capsule manifest",
        probe: Probe::AllMasked,
    },
    Invariant {
        name: b"HYBRID-PQ SIGNATURES",
        claim: b"every binary loaded at runtime is signed Ed25519 + ML-DSA-65 and chains to the baked trust anchor",
        mechanism: b"capsule_spawn::spawn_verified rejects any ELF whose nonos_id_cert + manifest do not both verify against BAKED_TRUST_ANCHOR_POLICY",
        probe: Probe::NotAtRuntime,
    },
    Invariant {
        name: b"ADMIN IS INIT ONLY",
        claim: b"no capsule other than init holds authority over the whole machine",
        mechanism: b"Admin is absent from every shipped CAPSULE_REQUIRED_CAPS but init's; the kernel refuses reboot and shutdown syscalls outside the mask",
        probe: Probe::OnlyInit(CAP_ADMIN),
    },
];
