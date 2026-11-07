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

//! The wire of the two Direct-only answers a session may ask for, pure, so
//! nsh_front_proofs holds them: an A lookup to `net.dns` and an ICMP echo
//! through `net.ip`, each frame built and each answer read with every
//! length checked.

use nonos_shell::host::{Echo, Resolve};

pub const HDR: usize = 20;
pub const DNS_MAGIC: u32 = 0x4E44_4E53;
pub const OP_RESOLVE_A: u16 = 2;
pub const IP_MAGIC: u32 = 0x4E49_5034;
pub const OP_SEND_PACKET: u16 = 4;
pub const OP_POLL_PACKET: u16 = 5;
pub const PROTO_ICMP: u8 = 1;
/// The echo identifier nsh.front's echoes carry, apart from the terminal's.
pub const ECHO_ID: u16 = 0x4E53;
pub const PAYLOAD: [u8; 16] = *b"NONOS-nsh-echo!!";
/// An ICMP echo header and PAYLOAD.
pub const ECHO_LEN: usize = 8 + PAYLOAD.len();

/// What net.core's `net.dns` answers a lookup that leaves in the clear with
/// off Direct, and what the session is told.
pub const DNS_NOT_DIRECT: u16 = 12;
pub const NET_CORE_REFUSED: &str = "net.core refuses a lookup in the clear off the Direct route";
const DNS_TIMEOUT: u16 = 6;
const DNS_NO_LEASE: u16 = 11;

const IP_NO_CONFIG: u16 = 5;
const IP_NO_ROUTE: u16 = 6;
const IP_NO_NEIGHBOUR: u16 = 7;

/// A request header for `magic` and `op` with a body of `len` bytes.
pub fn header(magic: u32, op: u16, len: usize) -> Option<[u8; HDR]> {
    let mut h = [0u8; HDR];
    h[0..4].copy_from_slice(&magic.to_le_bytes());
    h[4..6].copy_from_slice(&1u16.to_le_bytes());
    h[6..8].copy_from_slice(&op.to_le_bytes());
    h[12..16].copy_from_slice(&1u32.to_le_bytes());
    h[16..20].copy_from_slice(&u32::try_from(len).ok()?.to_le_bytes());
    Some(h)
}

/// The errno and body of an answer `got` bytes long in `rx`, or None when it
/// is shorter than a header or claims more body than arrived.
pub fn answer(rx: &[u8], got: i64) -> Option<(u16, &[u8])> {
    let got = usize::try_from(got).ok().filter(|n| *n >= HDR && *n <= rx.len())?;
    let len = usize::try_from(u32::from_le_bytes([rx[16], rx[17], rx[18], rx[19]])).ok()?;
    let end = HDR.checked_add(len).filter(|e| *e <= got)?;
    Some((u16::from_le_bytes([rx[8], rx[9]]), &rx[HDR..end]))
}

/// What a lookup's answer comes to; None when `net.dns` did not answer.
pub fn resolved(answer: Option<(u16, &[u8])>) -> Resolve {
    match answer {
        Some((0, &[a, b, c, d, ..])) => Resolve::Addr([a, b, c, d]),
        Some((DNS_NOT_DIRECT, _)) => Resolve::Refused(NET_CORE_REFUSED),
        None | Some((DNS_TIMEOUT | DNS_NO_LEASE, _)) => Resolve::NoDns,
        Some(_) => Resolve::NoName,
    }
}

/// An echo request numbered `seq`, its checksum set.
pub fn echo_request(seq: u16) -> [u8; ECHO_LEN] {
    let mut m = [0u8; ECHO_LEN];
    m[0] = 8;
    m[4..6].copy_from_slice(&ECHO_ID.to_be_bytes());
    m[6..8].copy_from_slice(&seq.to_be_bytes());
    m[8..].copy_from_slice(&PAYLOAD);
    let ck = checksum(&m);
    m[2..4].copy_from_slice(&ck.to_be_bytes());
    m
}

/// The internet checksum of `data` (RFC 1071).
pub fn checksum(data: &[u8]) -> u16 {
    let mut sum: u32 = data
        .chunks(2)
        .map(|c| u32::from(u16::from_be_bytes([c[0], c.get(1).copied().unwrap_or(0)])))
        .fold(0u32, |s, w| s.wrapping_add(w));
    while sum >> 16 != 0 {
        sum = (sum & 0xffff) + (sum >> 16);
    }
    !(sum as u16)
}

/// What sending an echo came to, from `net.ip`'s errno.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sent {
    Yes,
    /// The next hop is still being resolved: send again.
    Again,
    Stop(Echo),
}

pub fn sent(errno: Option<u16>) -> Sent {
    match errno {
        Some(0) => Sent::Yes,
        Some(IP_NO_NEIGHBOUR) => Sent::Again,
        Some(IP_NO_ROUTE) => Sent::Stop(Echo::NoRoute),
        Some(IP_NO_CONFIG) => Sent::Stop(Echo::NotReady),
        Some(_) => Sent::Stop(Echo::Failed),
        None => Sent::Stop(Echo::Unavailable),
    }
}

/// Whether a payload `net.ip` handed over is the reply to echo `seq` sent
/// to `dst`: from `dst`, ICMP, an echo reply, our identifier and `seq`.
/// The body is source[4], destination[4], protocol, then the ICMP message.
pub fn is_reply(body: &[u8], dst: [u8; 4], seq: u16) -> bool {
    match body {
        [s0, s1, s2, s3, _, _, _, _, PROTO_ICMP, 0, _, _, _, i0, i1, q0, q1, ..] => {
            [*s0, *s1, *s2, *s3] == dst
                && u16::from_be_bytes([*i0, *i1]) == ECHO_ID
                && u16::from_be_bytes([*q0, *q1]) == seq
        }
        _ => false,
    }
}
