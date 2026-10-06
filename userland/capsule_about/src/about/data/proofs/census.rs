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

//! Who holds authority on this machine, and whether the spawn gate admitted
//! each of them. Pure: the inputs are the process table and the attestation
//! registry as read, so the rules are proven on the host (attest_doc_proofs).
//!
//! The claim it settles is the one that matters for a capability system:
//! every process that holds any capability was admitted by the kernel's gate.
//! Two kinds of process are outside the registry by design and are named
//! rather than hidden. init is the kernel's own. A Linux guest is born with an
//! empty mask (`MkForeignSpawn`) and acts only through the personality that
//! supervises it, which is itself admitted; a capability-free process can
//! reach nothing, so it is counted as sandboxed, not as unproven.

/// The registry's authority byte for a capsule that ran on a publisher's
/// signature with no proof behind it.
pub const AUTHORITY_PUBLISHER: u8 = 255;
/// The Network capability (abi/caps.toml).
pub const CAP_NETWORK: u64 = 1 << 2;
/// A process this young may be between creation and its registry entry.
pub const STARTING_MS: u64 = 1_000;

const STATE_NEW: u8 = 0;
const STATE_ZOMBIE: u8 = 5;
const INIT: &[u8] = b"init";

/// One process table row, as much of it as the census needs.
#[derive(Clone, Copy)]
pub struct Proc<'a> {
    pub pid: u32,
    pub state: u8,
    pub uptime_ms: u64,
    pub caps: u64,
    pub name: &'a [u8],
}

/// One registry entry.
#[derive(Clone, Copy)]
pub struct Admitted {
    pub pid: u32,
    pub caps: u64,
    pub authority: u8,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Census {
    /// Live processes counted (not new, not exited).
    pub running: u32,
    /// Of those, admitted by the gate.
    pub admitted: u32,
    /// Admitted under a proof against the vendor's tree (authority 0).
    pub vendor: u32,
    /// Admitted under a proof against a root enrolled on this machine.
    pub enrolled: u32,
    /// Admitted on a publisher's signature alone.
    pub signed_only: u32,
    /// Capability-free processes outside the registry: hosted guests.
    pub sandboxed: u32,
    /// init.
    pub kernel: u32,
    /// Too young to have been recorded yet; neither passed nor failed.
    pub starting: u32,
    /// Holding a capability with no admission record. Must be zero.
    pub unadmitted: u32,
    /// Admitted capsules that hold Network.
    pub network: u32,
    /// The first unadmitted pid, for the screen to name.
    pub first_unadmitted: Option<u32>,
}

pub fn census(procs: &[Proc<'_>], admitted: &[Admitted]) -> Census {
    let mut c = Census::default();
    for p in procs {
        if p.state == STATE_NEW || p.state == STATE_ZOMBIE {
            continue;
        }
        c.running += 1;
        if let Some(a) = admitted.iter().find(|a| a.pid == p.pid) {
            c.admitted += 1;
            match a.authority {
                0 => c.vendor += 1,
                AUTHORITY_PUBLISHER => c.signed_only += 1,
                _ => c.enrolled += 1,
            }
            if a.caps & CAP_NETWORK != 0 {
                c.network += 1;
            }
        } else if p.name == INIT {
            c.kernel += 1;
        } else if p.caps == 0 {
            c.sandboxed += 1;
        } else if p.uptime_ms < STARTING_MS {
            c.starting += 1;
        } else {
            c.unadmitted += 1;
            if c.first_unadmitted.is_none() {
                c.first_unadmitted = Some(p.pid);
            }
        }
    }
    c
}
