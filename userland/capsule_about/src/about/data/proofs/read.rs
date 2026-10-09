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

//! One sample of everything the board shows.

use alloc::vec;
use alloc::vec::Vec;
use core::ptr;

use nonos_libc::{
    mk_attest_entries, mk_ipc_call_timeout, mk_service_lookup, ATTEST_ENTRY_LEN, PROC_NAME_LEN,
};
use nonos_policy_proto::Field;
use nonos_route_proof::{
    ask_frame, read_answer, reply_body, route_verdict, Latest, RouteVerdict, ANSWER_FRAME_LEN,
    ATTEST_SERVICE, OP_PROOF_ROUTE,
};

use super::census::{census, Admitted, Census, Proc, CAP_NETWORK};
use super::session::{Boot, Mark};
use crate::about::data::verify::{each, name_of, recorded, Verdict};

const MAX_ENTRIES: usize = 256;
const BOARD_REPLY_MS: u64 = 100;

/// An admitted capsule that can reach the network.
#[derive(Clone, Copy)]
pub struct Holder {
    pub pid: u32,
    pub name: [u8; PROC_NAME_LEN],
    pub name_len: usize,
    pub measurement: [u8; 32],
    pub authority: u8,
}

pub struct Snapshot {
    pub boot: Boot,
    /// `None` when the kernel would not show the table or the registry.
    pub census: Option<Census>,
    pub holders: Vec<Holder>,
    /// The policy store's route, `None` when it did not answer.
    pub chosen: Option<u8>,
    /// The board's answer, `None` when the attest service did not answer.
    pub board: Option<(Latest, Latest)>,
    pub route: RouteVerdict,
}

struct Row {
    pid: u32,
    state: u8,
    uptime_ms: u64,
    caps: u64,
    name: [u8; PROC_NAME_LEN],
    len: usize,
}

pub fn read() -> Snapshot {
    let boot = boot();
    let mut rows: Vec<Row> = Vec::new();
    let table = each(|e| {
        let name = name_of(e);
        let mut n = [0u8; PROC_NAME_LEN];
        n[..name.len()].copy_from_slice(name);
        rows.push(Row {
            pid: e.pid,
            state: e.state,
            uptime_ms: e.uptime_ms,
            caps: e.caps,
            name: n,
            len: name.len(),
        });
    });
    // The registry is read after the table, so a capsule in the table was
    // spawned, and recorded, before the registry read began.
    let registry = entries();
    let (census, holders) = match (table, registry) {
        (true, Some(reg)) => {
            let procs: Vec<Proc<'_>> = rows
                .iter()
                .map(|r| Proc {
                    pid: r.pid,
                    state: r.state,
                    uptime_ms: r.uptime_ms,
                    caps: r.caps,
                    name: &r.name[..r.len],
                })
                .collect();
            let admitted: Vec<Admitted> = reg.iter().map(|e| e.0).collect();
            let holders = reg
                .iter()
                .filter(|(a, _)| a.caps & CAP_NETWORK != 0)
                .map(|(a, m)| holder(a, m, &rows))
                .collect();
            (Some(census(&procs, &admitted)), holders)
        }
        _ => (None, Vec::new()),
    };
    let chosen = chosen_route();
    let board = board();
    let latest = match (chosen, board) {
        (Some(nonos_route_proof::ROUTE_NYM), Some((nym, _))) => nym,
        (Some(nonos_route_proof::ROUTE_ANYONE), Some((_, anyone))) => anyone,
        _ => None,
    };
    Snapshot { boot, census, holders, chosen, board, route: route_verdict(chosen, latest) }
}

fn boot() -> Boot {
    let mark = |v: Verdict| match v {
        Verdict::Holds => Mark::Holds,
        Verdict::Broken => Mark::Broken,
        Verdict::Unknown => Mark::Unknown,
    };
    match recorded() {
        Some(r) => Boot {
            signature: mark(r.kernel_signature),
            attestation: mark(r.attestation),
            proof: mark(r.proof),
        },
        None => Boot { signature: Mark::Unknown, attestation: Mark::Unknown, proof: Mark::Unknown },
    }
}

/* Pid big-endian, the measurement, the mask big-endian, the authority byte. */
fn entries() -> Option<Vec<(Admitted, [u8; 32])>> {
    let mut buf = vec![0u8; MAX_ENTRIES * ATTEST_ENTRY_LEN];
    let rc = mk_attest_entries(&mut buf);
    if rc < 0 || rc as usize > buf.len() || rc as usize % ATTEST_ENTRY_LEN != 0 {
        return None;
    }
    Some(
        buf[..rc as usize]
            .chunks_exact(ATTEST_ENTRY_LEN)
            .map(|e| {
                let mut m = [0u8; 32];
                m.copy_from_slice(&e[4..36]);
                let mut caps = [0u8; 8];
                caps.copy_from_slice(&e[36..44]);
                let a = Admitted {
                    pid: u32::from_be_bytes([e[0], e[1], e[2], e[3]]),
                    caps: u64::from_be_bytes(caps),
                    authority: e[ATTEST_ENTRY_LEN - 1],
                };
                (a, m)
            })
            .collect(),
    )
}

fn holder(a: &Admitted, measurement: &[u8; 32], rows: &[Row]) -> Holder {
    let (name, name_len) = match rows.iter().find(|r| r.pid == a.pid) {
        Some(r) => (r.name, r.len),
        None => ([0u8; PROC_NAME_LEN], 0),
    };
    Holder { pid: a.pid, name, name_len, measurement: *measurement, authority: a.authority }
}

fn chosen_route() -> Option<u8> {
    let port = nonos_policy_client::lookup()?;
    nonos_policy_client::get_u8(port, Field::NetworkRoute)
}

fn board() -> Option<(Latest, Latest)> {
    let mut port = 0u32;
    let rc = mk_service_lookup(
        ATTEST_SERVICE.as_ptr(),
        ATTEST_SERVICE.len(),
        &mut port as *mut u32,
        ptr::null_mut(),
    );
    if rc != 0 || port == 0 {
        return None;
    }
    let ask = ask_frame(1);
    let mut reply = [0u8; ANSWER_FRAME_LEN];
    let n = mk_ipc_call_timeout(
        port as u64,
        ask.as_ptr(),
        ask.len(),
        reply.as_mut_ptr(),
        reply.len(),
        BOARD_REPLY_MS,
    );
    if n <= 0 {
        return None;
    }
    let got = &reply[..(n as usize).min(reply.len())];
    match reply_body(got, OP_PROOF_ROUTE, 1)? {
        (0, body) => read_answer(body),
        _ => None,
    }
}
