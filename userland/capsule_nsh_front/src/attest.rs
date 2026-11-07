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

//! The bridge the server attests through. `nsh_server` asks for a document
//! over a per-handshake challenge; this capsule holds Network, so the kernel
//! refuses to quote to it. The ask is relayed to nsh.attest, which holds
//! AttestRead and no Network, and returns the machine's real NONOSATT v5
//! document, bound to that very challenge. Nothing here invents or reshapes
//! attestation: it is a transport to the one capsule allowed to produce it.

use alloc::vec;
use alloc::vec::Vec;
use nonos_libc::{mk_debug, mk_uptime_ms};
use nsh_front_wire::{OP_DOC, OP_ENTRIES};
use nsh_server::{AttestFault, Attestation};

use crate::relay_said::{line, op_name, Fault};

/// The kernel's 1 MiB IPC message limit, the most a single relay may carry.
const RESP_CAP: usize = 1 << 20;
/// The refusal the kernel returns for a document it will not produce.
const REFUSED: i64 = -1;

/// The IPC endpoint of nsh.attest, resolved once at setup.
pub struct AttestBridge {
    port: u64,
    resp: Vec<u8>,
}

impl AttestBridge {
    pub fn new(port: u32) -> Self {
        AttestBridge { port: port as u64, resp: vec![0u8; RESP_CAP] }
    }

    /// Relay one op to nsh.attest and return its body, or a fault. A fault
    /// is said on the serial console (relay_said.rs): the client hears only
    /// BUSY.
    fn call(&mut self, op: u16, body: &[u8]) -> Result<Vec<u8>, AttestFault> {
        let start = mk_uptime_ms();
        self.relay(op, body).map_err(|(fault, said)| {
            let waited = u64::try_from(mk_uptime_ms().wrapping_sub(start)).unwrap_or(0);
            let text = line(op_name(op), said, waited);
            let _ = mk_debug(text.as_ptr(), text.len());
            fault
        })
    }

    fn relay(&mut self, op: u16, body: &[u8]) -> Result<Vec<u8>, (AttestFault, Fault)> {
        crate::relay::call(self.port, op, body, &mut self.resp, 0).map_err(|f| match f {
            Fault::NoAnswer(n) => (AttestFault::Kernel(n), f),
            Fault::Status(
                nsh_front_wire::ST_ATTEST_REFUSED
                | nsh_front_wire::ST_REFUSED_CALLER
                | nsh_front_wire::ST_NO_TPM,
            ) => (AttestFault::Kernel(REFUSED), f),
            Fault::Status(_) | Fault::Unframed => (AttestFault::Malformed, f),
        })
    }
}

impl Attestation for AttestBridge {
    fn document(&mut self, challenge: &[u8; 32]) -> Result<Vec<u8>, AttestFault> {
        self.call(OP_DOC, challenge)
    }

    fn entries(&mut self) -> Result<Vec<u8>, AttestFault> {
        self.call(OP_ENTRIES, &[])
    }
}
