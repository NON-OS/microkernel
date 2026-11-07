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

//! What the shared shell may ask of this machine, from nsh.front's own
//! grant: randomness and the clocks, and the memory figures every capsule
//! may read, and the measured-boot state any token may read (MkAttestStatus,
//! not the TPM quote, which only nsh.attest asks for), and the service
//! registry, which its IPC grant reads (`service`), and, under Network and
//! only while the chosen network is Direct, a name lookup and an ICMP echo
//! (net_ask.rs): off Direct neither is asked for and the session is told
//! why, the rule nonos_route_link holds for every capsule. nsh.front holds no
//! filesystem capability, so a file is never read and `diff` says so; the
//! shell fails closed rather than reach.

use alloc::vec::Vec;
use nonos_libc::{mem_kb, mk_attest_status, mk_service_lookup, mk_uptime_ms, AttestStatus};
use nonos_shell::host::{Attest, Echo, Host, Mem, Proc, Resolve, Service};

use crate::{clock, net_ask, procs, rng};

pub struct FrontHost;

impl Host for FrontHost {
    fn random(&mut self, buf: &mut [u8]) -> bool {
        rng::fill(buf)
    }

    fn wall_ms(&mut self) -> Option<u64> {
        Some(clock::now()).filter(|ms| *ms > 0)
    }

    fn uptime_ms(&mut self) -> Option<u64> {
        u64::try_from(mk_uptime_ms()).ok()
    }

    fn mem(&mut self) -> Option<Mem> {
        let (total, free) = mem_kb()?;
        Some(Mem { total: total.checked_mul(1024)?, used: (total - free).checked_mul(1024)? })
    }

    fn procs(&mut self) -> Option<Vec<Proc>> {
        procs::table()
    }

    fn read_file(&mut self, _: &[u8], _: usize) -> Option<Vec<u8>> {
        None
    }

    fn attest(&mut self) -> Option<Attest> {
        let mut att = AttestStatus::default();
        if mk_attest_status(&mut att) != 0 {
            return None;
        }
        Some(Attest {
            kernel_sig_ok: att.kernel_sig_ok == 1,
            secure_boot: att.secure_boot == 1,
            attested: att.zk_attestation_ok == 1,
            proof: att.zk_verified,
            kernel_blake3: att.kernel_blake3,
            program_hash: att.program_hash,
        })
    }

    fn service(&mut self, name: &[u8]) -> Service {
        let (mut port, mut pid) = (0u32, 0u32);
        match mk_service_lookup(name.as_ptr(), name.len(), &mut port, &mut pid) {
            0 if port != 0 => Service::Found { port, pid },
            0 | ENOENT => Service::Absent,
            _ => Service::Unanswered,
        }
    }

    fn resolve(&mut self, host: &[u8]) -> Resolve {
        match nonos_route_link::direct_refusal() {
            Some(why) => Resolve::Refused(why),
            None => net_ask::resolve(host),
        }
    }

    fn echo(&mut self, dst: [u8; 4], seq: u16) -> Echo {
        match nonos_route_link::direct_refusal() {
            Some(why) => Echo::Refused(why),
            None => net_ask::echo(dst, seq),
        }
    }
}

/// The registry holds no service of the name asked for.
const ENOENT: i64 = -2;
