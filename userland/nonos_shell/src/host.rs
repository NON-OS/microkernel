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

//! What a builtin may ask of the machine it runs on, beyond writing lines:
//! randomness, the clocks, memory figures, the process table, the service
//! registry, reading a file, and on the Direct route a name lookup and an
//! ICMP echo. The GUI terminal answers from the kernel under its own
//! capabilities; a headless session answers from whatever its capsule holds. Every answer may be
//! None: a builtin whose host cannot answer says so and fails, it never
//! guesses.

use alloc::vec::Vec;

/// Memory figures, in bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mem {
    pub total: u64,
    pub used: u64,
}

/// One row of the machine's process table, as the kernel shows it to the
/// host: no counter, no capability, only who it is and what it is doing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Proc {
    pub pid: u32,
    pub ppid: u32,
    /// The kernel's state number (`ps` names it).
    pub state: u8,
    pub name: Vec<u8>,
}

/// What the kernel recorded of this boot's measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Attest {
    /// The loader checked the kernel's signature.
    pub kernel_sig_ok: bool,
    pub secure_boot: bool,
    /// The kernel's attestation check passed.
    pub attested: bool,
    /// `PROOF_STARK`, `PROOF_PATH_ONLY`, or anything else for none recorded.
    pub proof: u8,
    pub kernel_blake3: [u8; 32],
    pub program_hash: [u8; 32],
}

/// What the kernel's service registry answered for one name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Service {
    /// Registered: the port its requests go to and the pid that serves it.
    Found { port: u32, pid: u32 },
    /// The registry holds no service of that name.
    Absent,
    /// The host could not ask the registry.
    Unanswered,
}

/// The loader checked the kernel's STARK proof.
pub const PROOF_STARK: u8 = 1;
/// A development kernel, checked on its Merkle path alone.
pub const PROOF_PATH_ONLY: u8 = 2;

/// What looking a name up in the clear came to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resolve {
    Addr([u8; 4]),
    /// Not sent: the chosen network is not Direct, for this reason.
    Refused(&'static str),
    /// The DNS servers answered that the name has no address.
    NoName,
    /// No DNS server answered.
    NoDns,
    /// This host cannot look names up.
    Unavailable,
}

/// What one ICMP echo came to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Echo {
    /// Answered, after this many milliseconds.
    Reply(u64),
    /// Sent, and not answered in time.
    Timeout,
    /// Never sent: the next hop did not answer ARP in time.
    Unreachable,
    NoRoute,
    /// The interface has no address yet.
    NotReady,
    /// The stack refused to send it.
    Failed,
    /// Not sent: the chosen network is not Direct, for this reason.
    Refused(&'static str),
    /// This host cannot send echoes.
    Unavailable,
}

pub trait Host {
    /// Fill `buf` from the kernel's generator; false if it would not.
    fn random(&mut self, buf: &mut [u8]) -> bool;
    /// Milliseconds since the Unix epoch, if the machine has a wall clock.
    fn wall_ms(&mut self) -> Option<u64>;
    /// Milliseconds since boot.
    fn uptime_ms(&mut self) -> Option<u64>;
    /// The machine's memory, if the host may read it.
    fn mem(&mut self) -> Option<Mem>;
    /// The machine's processes, if the host may read the table.
    fn procs(&mut self) -> Option<Vec<Proc>>;
    /// A file's bytes, read whole and held to `max`; None if it cannot be
    /// read or is longer.
    fn read_file(&mut self, path: &[u8], max: usize) -> Option<Vec<u8>>;
    /// This boot's measured state, if the kernel recorded one. A host that
    /// cannot read it keeps the default and `attest` says so.
    fn attest(&mut self) -> Option<Attest> {
        None
    }
    /// Look `name` up in the kernel's service registry. `name` is already
    /// held to `builtin::service::NAME_MAX` bytes of printable ASCII. A host
    /// that cannot ask keeps the default and `service` says so.
    fn service(&mut self, _name: &[u8]) -> Service {
        Service::Unanswered
    }
    /// The IPv4 address `host` names, asked of the machine's resolver. Only
    /// on the Direct route: a lookup leaves in the clear.
    fn resolve(&mut self, _host: &[u8]) -> Resolve {
        Resolve::Unavailable
    }
    /// One ICMP echo to `dst`, numbered `seq`, waited for a bounded time.
    /// Only on the Direct route: ICMP crosses no anonymity network.
    fn echo(&mut self, _dst: [u8; 4], _seq: u16) -> Echo {
        Echo::Unavailable
    }
}
