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

//! The network on the other side of net.ip, played on the host.
//!
//! The capsule's IP client calls arrive here through the stand-in nonos_libc:
//! a poll takes the next segment a proof queued, a send is kept for the proof
//! to read. The peer can answer a handshake by itself, so a connect or an
//! accept completes the way it would against a real host.

use std::collections::VecDeque;
use std::sync::{Mutex, MutexGuard};

use crate::protocol::{IPC_PAYLOAD_MAX, MAGIC, OP_ACCEPT, OP_CLOSE, OP_CONNECT, OP_LISTEN};
use crate::protocol::{OP_RECV, OP_SEND, OP_STATE};
use crate::server::handlers;
use crate::server::parse_req::{parse, HDR_LEN};

pub const IP_PORT: u32 = 4402;
pub const LOCAL: [u8; 4] = [10, 0, 2, 15];
pub const REMOTE: [u8; 4] = [93, 184, 216, 34];
pub const APP: u32 = 77;
pub const PEER_ISS: u32 = 0x7000_0000;
pub const PEER_PORT: u16 = 443;

pub const FIN: u8 = 0x01;
pub const SYN: u8 = 0x02;
pub const RST: u8 = 0x04;
pub const PSH: u8 = 0x08;
pub const ACK: u8 = 0x10;

const IP_MAGIC: u32 = 0x4E49_5034;
const OP_GET_CONFIG: u16 = 2;
const OP_SEND_PACKET: u16 = 4;
const OP_POLL_PACKET: u16 = 5;
const E_RX_EMPTY: u16 = 10;

/// One segment as it crossed the wire.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Seg {
    pub src: [u8; 4],
    pub dst: [u8; 4],
    pub sport: u16,
    pub dport: u16,
    pub seq: u32,
    pub ack: u32,
    pub flags: u8,
    pub window: u16,
    pub options: Vec<u8>,
    pub payload: Vec<u8>,
}

impl Seg {
    /// A segment from the remote host to the capsule's address.
    pub fn from_peer(dport: u16, seq: u32, ack: u32, flags: u8, payload: &[u8]) -> Self {
        Seg {
            src: REMOTE,
            dst: LOCAL,
            sport: PEER_PORT,
            dport,
            seq,
            ack,
            flags,
            window: 65535,
            options: Vec::new(),
            payload: payload.to_vec(),
        }
    }

    pub fn has(&self, flag: u8) -> bool {
        self.flags & flag != 0
    }
}

/// How the peer answers on its own.
#[derive(Clone, Copy)]
pub struct Auto {
    /// Answer a SYN with a SYN-ACK.
    pub answer_syn: bool,
    /// Answer a SYN-ACK with the ACK that completes a passive open.
    pub answer_synack: bool,
    /// The MSS option the peer puts in its SYN-ACK, if any.
    pub mss: Option<u16>,
    /// Refuse a SYN with RST and ACK, the RST carrying this sequence number.
    pub refuse: Option<u32>,
    /// Before answering a SYN, send a bare RST (no ACK) with this sequence
    /// number, as an off-path sender racing the real answer would.
    pub forged_rst: Option<u32>,
}

impl Auto {
    /// Answers both handshakes, with no options and nothing forged.
    pub const fn plain() -> Self {
        Auto { answer_syn: true, answer_synack: true, mss: None, refuse: None, forged_rst: None }
    }
}

struct Wire {
    /// Source, destination and the exact segment bytes, in arrival order.
    inbound: VecDeque<([u8; 4], [u8; 4], Vec<u8>)>,
    outbound: Vec<Seg>,
    auto: Auto,
}

static WIRE: Mutex<Wire> = Mutex::new(Wire {
    inbound: VecDeque::new(),
    outbound: Vec::new(),
    auto: Auto::plain(),
});

fn wire() -> MutexGuard<'static, Wire> {
    WIRE.lock().unwrap_or_else(|e| e.into_inner())
}

/// RFC 1071 over the IPv4 pseudo-header and the segment, written apart from
/// the capsule's own so the two check each other.
pub fn checksum(src: &[u8; 4], dst: &[u8; 4], segment: &[u8]) -> u16 {
    let mut sum: u32 = 0;
    let mut add = |hi: u8, lo: u8| sum += u32::from(u16::from_be_bytes([hi, lo]));
    add(src[0], src[1]);
    add(src[2], src[3]);
    add(dst[0], dst[1]);
    add(dst[2], dst[3]);
    add(0, 6);
    let len = segment.len() as u16;
    add((len >> 8) as u8, len as u8);
    for pair in segment.chunks(2) {
        add(pair[0], *pair.get(1).unwrap_or(&0));
    }
    while sum >> 16 != 0 {
        sum = (sum & 0xFFFF) + (sum >> 16);
    }
    !(sum as u16)
}

/// The segment's wire bytes, checksum sealed.
pub fn encode(s: &Seg) -> Vec<u8> {
    let mut options = s.options.clone();
    while !options.len().is_multiple_of(4) {
        options.push(0);
    }
    let words = (20 + options.len()) / 4;
    let mut out = Vec::with_capacity(words * 4 + s.payload.len());
    out.extend_from_slice(&s.sport.to_be_bytes());
    out.extend_from_slice(&s.dport.to_be_bytes());
    out.extend_from_slice(&s.seq.to_be_bytes());
    out.extend_from_slice(&s.ack.to_be_bytes());
    out.push((words as u8) << 4);
    out.push(s.flags);
    out.extend_from_slice(&s.window.to_be_bytes());
    out.extend_from_slice(&[0, 0, 0, 0]);
    out.extend_from_slice(&options);
    out.extend_from_slice(&s.payload);
    let ck = checksum(&s.src, &s.dst, &out);
    out[16..18].copy_from_slice(&ck.to_be_bytes());
    out
}

fn decode(src: [u8; 4], dst: [u8; 4], b: &[u8]) -> Seg {
    assert!(b.len() >= 20, "the capsule sent a short segment");
    assert_eq!(checksum(&src, &dst, b), 0, "the capsule's checksum verifies");
    let be16 = |i: usize| u16::from_be_bytes([b[i], b[i + 1]]);
    let be32 = |i: usize| u32::from_be_bytes([b[i], b[i + 1], b[i + 2], b[i + 3]]);
    let hl = usize::from(b[12] >> 4) * 4;
    Seg {
        src,
        dst,
        sport: be16(0),
        dport: be16(2),
        seq: be32(4),
        ack: be32(8),
        flags: b[13],
        window: be16(14),
        options: b[20..hl].to_vec(),
        payload: b[hl..].to_vec(),
    }
}

fn header(op: u16, errno: u16, len: usize) -> Vec<u8> {
    let mut out = Vec::with_capacity(20 + len);
    out.extend_from_slice(&IP_MAGIC.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&op.to_le_bytes());
    out.extend_from_slice(&errno.to_le_bytes());
    out.extend_from_slice(&[0, 0, 0, 0, 0, 0]);
    out.extend_from_slice(&(len as u32).to_le_bytes());
    out
}

fn answer_on_its_own(w: &mut Wire, s: &Seg) {
    let syn_only = s.has(SYN) && !s.has(ACK);
    let synack = s.has(SYN) && s.has(ACK);
    if syn_only {
        if let Some(seq) = w.auto.forged_rst {
            let mut rst = Seg::from_peer(s.sport, seq, 0, RST, &[]);
            rst.sport = s.dport;
            w.inbound.push_back((rst.src, rst.dst, encode(&rst)));
        }
        if let Some(seq) = w.auto.refuse {
            let mut rst = Seg::from_peer(s.sport, seq, s.seq.wrapping_add(1), RST | ACK, &[]);
            rst.sport = s.dport;
            w.inbound.push_back((rst.src, rst.dst, encode(&rst)));
            return;
        }
    }
    if syn_only && w.auto.answer_syn {
        let mut reply = Seg::from_peer(s.sport, PEER_ISS, s.seq.wrapping_add(1), SYN | ACK, &[]);
        reply.sport = s.dport;
        if let Some(mss) = w.auto.mss {
            reply.options = vec![2, 4, (mss >> 8) as u8, mss as u8];
        }
        w.inbound.push_back((reply.src, reply.dst, encode(&reply)));
    }
    if synack && w.auto.answer_synack {
        let mut reply = Seg::from_peer(s.sport, s.ack, s.seq.wrapping_add(1), ACK, &[]);
        reply.sport = s.dport;
        w.inbound.push_back((reply.src, reply.dst, encode(&reply)));
    }
}

/// net.ip, as the capsule's IP client sees it.
fn ip_service(endpoint: u64, req: &[u8]) -> Vec<u8> {
    assert_eq!(endpoint, u64::from(IP_PORT), "the capsule only calls net.ip");
    let op = u16::from_le_bytes([req[6], req[7]]);
    let mut w = wire();
    match op {
        OP_POLL_PACKET => match w.inbound.pop_front() {
            Some((src, dst, seg)) => {
                let mut out = header(op, 0, 9 + seg.len());
                out.extend_from_slice(&src);
                out.extend_from_slice(&dst);
                out.push(6);
                out.extend_from_slice(&seg);
                out
            }
            None => header(op, E_RX_EMPTY, 0),
        },
        OP_SEND_PACKET => {
            let mut dst = [0u8; 4];
            dst.copy_from_slice(&req[20..24]);
            assert_eq!(req[24], 6, "TCP rides protocol 6");
            let s = decode(LOCAL, dst, &req[25..]);
            answer_on_its_own(&mut w, &s);
            w.outbound.push(s);
            header(op, 0, 0)
        }
        OP_GET_CONFIG => {
            let mut out = header(op, 0, 17);
            let mut body = [0u8; 17];
            body[6..10].copy_from_slice(&LOCAL);
            out.extend_from_slice(&body);
            out
        }
        _ => header(op, 3, 0),
    }
}

/// A clean capsule and wire for one proof. Hold the guard for the proof's
/// whole run.
pub fn fresh() -> MutexGuard<'static, ()> {
    let guard = nonos_libc::serial();
    reset();
    guard
}

/// Clear the capsule and the wire, for a proof already holding `fresh()`.
pub fn reset() {
    {
        let mut t = crate::state::TABLE.lock();
        let handles: Vec<u32> = t.entries_mut().iter().map(|e| e.handle).collect();
        for h in handles {
            t.remove_by_handle(h);
        }
        t.timers = crate::state::Timers::new();
        t.seed_iss([0x0123_4567_89AB_CDEF, 0xFEDC_BA98_7654_3210]);
    }
    crate::state::set_ip_port(IP_PORT);
    crate::state::set_local_ip(LOCAL);
    nonos_libc::set_time(1_000_000);
    nonos_libc::revive_all();
    nonos_libc::set_responder(ip_service);
    let _ = nonos_libc::take_replies();
    let mut w = wire();
    w.inbound.clear();
    w.outbound.clear();
    w.auto = Auto::plain();
}

pub fn set_auto(auto: Auto) {
    wire().auto = auto;
}

/// Queue a segment for the capsule's next poll.
pub fn inject(s: Seg) {
    let bytes = encode(&s);
    wire().inbound.push_back((s.src, s.dst, bytes));
}

/// Queue segment bytes exactly as given, for shapes `Seg` cannot say.
pub fn inject_raw(src: [u8; 4], dst: [u8; 4], bytes: &[u8]) {
    wire().inbound.push_back((src, dst, bytes.to_vec()));
}

/// Every segment the capsule has sent since the last call.
pub fn sent() -> Vec<Seg> {
    core::mem::take(&mut wire().outbound)
}

/// Segments queued for the capsule and not yet polled.
pub fn pending() -> usize {
    wire().inbound.len()
}

/// Run one request through the handler the server loop would have chosen,
/// and return the reply's status and body.
pub fn request(op: u16, body: &[u8]) -> (u16, Vec<u8>) {
    let mut frame = Vec::with_capacity(HDR_LEN + body.len());
    frame.extend_from_slice(&MAGIC.to_le_bytes());
    frame.extend_from_slice(&1u16.to_le_bytes());
    frame.extend_from_slice(&op.to_le_bytes());
    frame.extend_from_slice(&[0, 0, 0, 0]);
    frame.extend_from_slice(&9u32.to_le_bytes());
    frame.extend_from_slice(&(body.len() as u32).to_le_bytes());
    frame.extend_from_slice(body);
    let (req, body) = parse(&frame).expect("a well formed request");
    let mut tx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    match op {
        OP_LISTEN => handlers::listen::handle(APP, &req, body, &mut tx),
        OP_CONNECT => handlers::connect::handle(APP, &req, body, &mut tx),
        OP_ACCEPT => handlers::accept::handle(APP, &req, body, &mut tx),
        OP_SEND => handlers::send::handle(APP, &req, body, &mut tx),
        OP_RECV => handlers::recv::handle(APP, &req, body, &mut tx),
        OP_CLOSE => handlers::close::handle(APP, &req, body, &mut tx),
        OP_STATE => handlers::state::handle(APP, &req, body, &mut tx),
        _ => panic!("no handler for op {op}"),
    }
    let reply = nonos_libc::take_replies().pop().expect("the handler replied");
    let b = reply.bytes;
    let errno = u16::from_le_bytes([b[8], b[9]]);
    let len = u32::from_le_bytes([b[16], b[17], b[18], b[19]]) as usize;
    (errno, b[20..20 + len].to_vec())
}

/// Open a connection to the peer; the peer answers the SYN by itself.
pub fn connect() -> u32 {
    let mut body = REMOTE.to_vec();
    body.extend_from_slice(&PEER_PORT.to_le_bytes());
    let (errno, reply) = request(OP_CONNECT, &body);
    assert_eq!(errno, 0, "connect");
    u32::from_le_bytes([reply[0], reply[1], reply[2], reply[3]])
}

pub fn handle_body(handle: u32) -> Vec<u8> {
    handle.to_le_bytes().to_vec()
}

pub fn recv(handle: u32) -> (u16, Vec<u8>) {
    request(OP_RECV, &handle_body(handle))
}

pub fn send(handle: u32, data: &[u8]) -> u16 {
    let mut body = handle_body(handle);
    body.extend_from_slice(data);
    request(OP_SEND, &body).0
}

pub fn state_of(handle: u32) -> Option<u8> {
    let (errno, body) = request(OP_STATE, &handle_body(handle));
    (errno == 0).then(|| body[0])
}

/// The capsule's (snd.nxt, rcv.nxt, snd.una) for a connection.
pub fn numbers(handle: u32) -> (u32, u32, u32) {
    let mut t = crate::state::TABLE.lock();
    let e = t.by_handle_mut(handle).expect("the connection exists");
    (e.tcb.send.nxt, e.tcb.recv.nxt, e.tcb.send.una)
}

/// Let the capsule take every queued segment.
pub fn drain() {
    while pending() > 0 {
        crate::server::tcp_rx::drain_one();
    }
}
