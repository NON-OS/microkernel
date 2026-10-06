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

//! net.udp and the upstream resolver behind it, played on the host.
//!
//! A query the capsule sends is handed to the proof's script, which says what
//! comes back and from where; the capsule's next receives take those
//! datagrams in order.

use std::collections::VecDeque;
use std::sync::{Mutex, MutexGuard};

use crate::protocol::{IPC_PAYLOAD_MAX, MAGIC, OP_RESOLVE_A};
use crate::server::handlers;
use crate::server::parse_req::{parse, HDR_LEN};

pub const UDP_PORT: u32 = 4420;
pub const SERVER: [u8; 4] = [192, 0, 2, 53];
pub const APP: u32 = 55;

const UDP_MAGIC: u32 = 0x4E55_4450;
const OP_BIND: u16 = 2;
const OP_SEND: u16 = 4;
const OP_RECV: u16 = 5;
const E_RX_EMPTY: u16 = 8;

/// One query as it left the capsule.
#[derive(Clone)]
pub struct Query {
    pub dst: [u8; 4],
    pub dport: u16,
    pub bytes: Vec<u8>,
}

/// One datagram for the capsule to receive.
#[derive(Clone)]
pub struct Datagram {
    pub src: [u8; 4],
    pub sport: u16,
    pub payload: Vec<u8>,
}

impl Datagram {
    /// From the configured server's port 53.
    pub fn from_server(payload: Vec<u8>) -> Self {
        Datagram { src: SERVER, sport: 53, payload }
    }
}

type Script = Box<dyn FnMut(&Query) -> Vec<Datagram> + Send>;

struct Net {
    inbound: VecDeque<Datagram>,
    queries: Vec<Query>,
    script: Option<Script>,
}

static NET: Mutex<Net> = Mutex::new(Net { inbound: VecDeque::new(), queries: Vec::new(), script: None });

fn net() -> MutexGuard<'static, Net> {
    NET.lock().unwrap_or_else(|e| e.into_inner())
}

fn header(op: u16, errno: u16, len: usize) -> Vec<u8> {
    let mut out = Vec::with_capacity(20 + len);
    out.extend_from_slice(&UDP_MAGIC.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&op.to_le_bytes());
    out.extend_from_slice(&errno.to_le_bytes());
    out.extend_from_slice(&[0, 0, 0, 0, 0, 0]);
    out.extend_from_slice(&(len as u32).to_le_bytes());
    out
}

fn udp_service(endpoint: u64, req: &[u8]) -> Vec<u8> {
    assert_eq!(endpoint, u64::from(UDP_PORT), "the resolver only calls net.udp");
    let op = u16::from_le_bytes([req[6], req[7]]);
    let mut n = net();
    match op {
        OP_SEND => {
            let b = &req[20..];
            let q = Query {
                dst: [b[2], b[3], b[4], b[5]],
                dport: u16::from_le_bytes([b[6], b[7]]),
                bytes: b[8..].to_vec(),
            };
            let replies = match n.script.as_mut() {
                Some(script) => script(&q),
                None => Vec::new(),
            };
            n.inbound.extend(replies);
            n.queries.push(q);
            header(op, 0, 0)
        }
        OP_RECV => match n.inbound.pop_front() {
            Some(d) => {
                let mut out = header(op, 0, 6 + d.payload.len());
                out.extend_from_slice(&d.src);
                out.extend_from_slice(&d.sport.to_le_bytes());
                out.extend_from_slice(&d.payload);
                out
            }
            None => header(op, E_RX_EMPTY, 0),
        },
        OP_BIND => header(op, 0, 0),
        _ => header(op, 3, 0),
    }
}

/// A clean capsule and network for one proof; hold the guard throughout.
pub fn fresh() -> MutexGuard<'static, ()> {
    let guard = nonos_libc::serial();
    nonos_libc::set_time(1_000_000);
    nonos_libc::set_responder(udp_service);
    let _ = nonos_libc::take_replies();
    crate::state::set_udp_port(UDP_PORT);
    crate::state::set_upstream(SERVER);
    crate::state::CACHE.lock().tick(u64::MAX);
    let mut n = net();
    n.inbound.clear();
    n.queries.clear();
    n.script = None;
    guard
}

/// What the upstream answers each query with.
pub fn script(f: impl FnMut(&Query) -> Vec<Datagram> + Send + 'static) {
    net().script = Some(Box::new(f));
}

/// How many queries have gone out.
pub fn queries() -> usize {
    net().queries.len()
}

/// A response to `q`: its id and question echoed, then one A record for the
/// question name.
pub fn answer(q: &Query, ip: [u8; 4], ttl: u32) -> Vec<u8> {
    let mut m = q.bytes.clone();
    m[2] = 0x81;
    m[3] = 0x80;
    m[7] = 1;
    m.extend_from_slice(&[0xC0, 0x0C, 0, 1, 0, 1]);
    m.extend_from_slice(&ttl.to_be_bytes());
    m.extend_from_slice(&[0, 4]);
    m.extend_from_slice(&ip);
    m
}

/// Resolve `name` through the capsule's OP_RESOLVE_A handler.
pub fn resolve_a(name: &str) -> (u16, Option<[u8; 4]>) {
    let mut frame = Vec::new();
    frame.extend_from_slice(&MAGIC.to_le_bytes());
    frame.extend_from_slice(&1u16.to_le_bytes());
    frame.extend_from_slice(&OP_RESOLVE_A.to_le_bytes());
    frame.extend_from_slice(&[0, 0, 0, 0]);
    frame.extend_from_slice(&3u32.to_le_bytes());
    frame.extend_from_slice(&(name.len() as u32).to_le_bytes());
    frame.extend_from_slice(name.as_bytes());
    let (req, body) = parse(&frame).expect("a well formed request");
    let mut tx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    handlers::resolve_a::handle(APP, &req, body, &mut tx);
    let reply = nonos_libc::take_replies().pop().expect("the handler replied").bytes;
    let errno = u16::from_le_bytes([reply[8], reply[9]]);
    let ip = (errno == 0).then(|| [reply[20], reply[21], reply[22], reply[23]]);
    (errno, ip)
}
