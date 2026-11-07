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

//! The calls behind a session's `nslookup` and `ping`: one A lookup to
//! `net.dns`, and one ICMP echo through `net.ip` waited for ECHO_WAIT_MS.
//! nsh.front holds Network, which both services require; the Direct check
//! is made before either is asked (shell_host.rs), and net.core holds it
//! again for the lookup.

use alloc::vec::Vec;

use nonos_libc::{mk_idle_ms, mk_ipc_call_timeout, mk_service_lookup, mk_uptime_ms};
use nonos_shell::host::{Echo, Resolve};

use crate::net_rule::{
    answer, echo_request, header, is_reply, resolved, sent, Sent, DNS_MAGIC, ECHO_LEN, HDR,
    IP_MAGIC, OP_POLL_PACKET, OP_RESOLVE_A, OP_SEND_PACKET, PROTO_ICMP,
};

/// How long a lookup is waited for: net.core gives up on its servers first.
const DNS_WAIT_MS: u64 = 6_000;
/// How long one echo is waited for, sent or not.
const ECHO_WAIT_MS: i64 = 1_000;
/// How long one call to `net.ip` may take.
const IP_CALL_MS: u64 = 500;
/// The pause between polls for the reply.
const POLL_GAP_MS: u64 = 10;
/// The most a polled payload holds, as net.ip bounds it.
const POLL_MAX: usize = 512;

fn port_of(name: &[u8]) -> Option<u64> {
    let (mut port, mut pid) = (0u32, 0u32);
    let rc = mk_service_lookup(name.as_ptr(), name.len(), &mut port, &mut pid);
    (rc == 0 && port != 0).then_some(u64::from(port))
}

fn call(port: u64, req: &[u8], rx: &mut [u8], wait_ms: u64) -> Option<(u16, Vec<u8>)> {
    let got =
        mk_ipc_call_timeout(port, req.as_ptr(), req.len(), rx.as_mut_ptr(), rx.len(), wait_ms);
    answer(rx, got).map(|(errno, body)| (errno, body.to_vec()))
}

/// The A record `host` names.
pub fn resolve(host: &[u8]) -> Resolve {
    let Some(port) = port_of(b"net.dns") else { return Resolve::Unavailable };
    let Some(head) = header(DNS_MAGIC, OP_RESOLVE_A, host.len()) else { return Resolve::NoName };
    let req = [&head[..], host].concat();
    let mut rx = [0u8; HDR + 4];
    let got = call(port, &req, &mut rx, DNS_WAIT_MS);
    resolved(got.as_ref().map(|(e, b)| (*e, &b[..])))
}

/// One echo to `dst` numbered `seq`.
pub fn echo(dst: [u8; 4], seq: u16) -> Echo {
    let Some(port) = port_of(b"net.ip") else { return Echo::Unavailable };
    let t0 = mk_uptime_ms();
    let Some(head) = header(IP_MAGIC, OP_SEND_PACKET, 5 + ECHO_LEN) else { return Echo::Failed };
    let send = [&head[..], &dst, &[PROTO_ICMP], &echo_request(seq)].concat();
    let Some(head) = header(IP_MAGIC, OP_POLL_PACKET, 1) else { return Echo::Failed };
    let poll = [&head[..], &[PROTO_ICMP]].concat();
    let mut rx = alloc::vec![0u8; HDR + POLL_MAX];
    let mut out = false;
    while mk_uptime_ms().wrapping_sub(t0) < ECHO_WAIT_MS {
        if !out {
            match sent(call(port, &send, &mut rx, IP_CALL_MS).map(|(e, _)| e)) {
                Sent::Yes => out = true,
                Sent::Again => {}
                Sent::Stop(e) => return e,
            }
        }
        if out {
            if let Some((0, body)) = call(port, &poll, &mut rx, IP_CALL_MS) {
                if is_reply(&body, dst, seq) {
                    let ms = mk_uptime_ms().wrapping_sub(t0);
                    return Echo::Reply(u64::try_from(ms).unwrap_or(0));
                }
            }
        }
        let _ = mk_idle_ms(POLL_GAP_MS);
    }
    if out {
        Echo::Timeout
    } else {
        Echo::Unreachable
    }
}
