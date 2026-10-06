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

//! net.l2 below the IP capsule, played on the host: frames a proof queues
//! come up through the capsule's polls, and frames it sends down are kept.

use std::collections::VecDeque;
use std::sync::{Mutex, MutexGuard};

use core::sync::atomic::Ordering;

use crate::protocol::{IPC_PAYLOAD_MAX, MAGIC, OP_POLL_PACKET};
use crate::route::{Route, ROUTES};
use crate::server::handlers;
use crate::server::parse_req::{parse, HDR_LEN};
use crate::state::IFACE;

pub const L2_PORT: u32 = 4400;
pub const LOCAL: [u8; 4] = [10, 0, 2, 15];
pub const REMOTE: [u8; 4] = [10, 0, 2, 2];
pub const OUR_MAC: [u8; 6] = [2, 0, 0, 0, 0, 1];
pub const PEER_MAC: [u8; 6] = [2, 0, 0, 0, 0, 2];
pub const APP: u32 = 66;

const L2_MAGIC: u32 = 0x4E4C_3200;
const OP_GET_MAC: u16 = 2;
const OP_SEND_FRAME: u16 = 4;
const OP_POLL_FRAME: u16 = 5;
const OP_ARP_RESOLVE: u16 = 6;
const E_RX_EMPTY: u16 = 8;

struct Link {
    up: VecDeque<Vec<u8>>,
    down: Vec<Vec<u8>>,
}

static LINK: Mutex<Link> = Mutex::new(Link { up: VecDeque::new(), down: Vec::new() });

fn link() -> MutexGuard<'static, Link> {
    LINK.lock().unwrap_or_else(|e| e.into_inner())
}

fn header(op: u16, errno: u16, len: usize) -> Vec<u8> {
    let mut out = Vec::with_capacity(20 + len);
    out.extend_from_slice(&L2_MAGIC.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&op.to_le_bytes());
    out.extend_from_slice(&errno.to_le_bytes());
    out.extend_from_slice(&[0, 0, 0, 0, 0, 0]);
    out.extend_from_slice(&(len as u32).to_le_bytes());
    out
}

fn l2_service(endpoint: u64, req: &[u8]) -> Vec<u8> {
    assert_eq!(endpoint, u64::from(L2_PORT), "net.ip only calls net.l2");
    let op = u16::from_le_bytes([req[6], req[7]]);
    let mut l = link();
    match op {
        OP_POLL_FRAME => match l.up.pop_front() {
            Some(frame) => {
                let mut out = header(op, 0, frame.len());
                out.extend_from_slice(&frame);
                out
            }
            None => header(op, E_RX_EMPTY, 0),
        },
        OP_SEND_FRAME => {
            l.down.push(req[20..].to_vec());
            header(op, 0, 0)
        }
        OP_ARP_RESOLVE => {
            let mut out = header(op, 0, 6);
            out.extend_from_slice(&PEER_MAC);
            out
        }
        OP_GET_MAC => {
            let mut out = header(op, 0, 6);
            out.extend_from_slice(&OUR_MAC);
            out
        }
        _ => header(op, 3, 0),
    }
}

/// A configured interface on 10.0.2.0/24 and an empty link.
pub fn fresh() -> MutexGuard<'static, ()> {
    let guard = nonos_libc::serial();
    nonos_libc::set_responder(l2_service);
    let _ = nonos_libc::take_replies();
    *IFACE.ipv4.lock() = LOCAL;
    *IFACE.mac.lock() = OUR_MAC;
    IFACE.l2_service_port.store(L2_PORT, Ordering::Release);
    ROUTES.clear();
    let _ = ROUTES.install(Route { network: [10, 0, 2, 0], prefix: 24, gateway: None });
    let _ = ROUTES.install(Route { network: [0; 4], prefix: 0, gateway: Some(REMOTE) });
    while crate::state::pop_any().is_some() {}
    let mut l = link();
    l.up.clear();
    l.down.clear();
    guard
}

/// RFC 1071 over `bytes`.
pub fn checksum(bytes: &[u8]) -> u16 {
    let mut sum: u32 = 0;
    for pair in bytes.chunks(2) {
        sum += u32::from(u16::from_be_bytes([pair[0], *pair.get(1).unwrap_or(&0)]));
    }
    while sum >> 16 != 0 {
        sum = (sum & 0xFFFF) + (sum >> 16);
    }
    !(sum as u16)
}

/// An Ethernet frame carrying one IPv4 packet, header checksum sealed.
pub fn frame(src: [u8; 4], dst: [u8; 4], protocol: u8, payload: &[u8]) -> Vec<u8> {
    let mut f = Vec::new();
    f.extend_from_slice(&OUR_MAC);
    f.extend_from_slice(&PEER_MAC);
    f.extend_from_slice(&[0x08, 0x00]);
    let total = (20 + payload.len()) as u16;
    let mut ip = vec![0x45, 0, (total >> 8) as u8, total as u8, 0, 1, 0x40, 0, 64, protocol, 0, 0];
    ip.extend_from_slice(&src);
    ip.extend_from_slice(&dst);
    let ck = checksum(&ip);
    ip[10..12].copy_from_slice(&ck.to_be_bytes());
    f.extend_from_slice(&ip);
    f.extend_from_slice(payload);
    f
}

/// An ICMP echo request with this identifier, sequence and data.
pub fn echo_request(id: u16, seq: u16, data: &[u8]) -> Vec<u8> {
    let mut m = vec![8, 0, 0, 0];
    m.extend_from_slice(&id.to_be_bytes());
    m.extend_from_slice(&seq.to_be_bytes());
    m.extend_from_slice(data);
    let ck = checksum(&m);
    m[2..4].copy_from_slice(&ck.to_be_bytes());
    m
}

/// Queue a frame for the capsule's next poll of net.l2.
pub fn arrive(frame: Vec<u8>) {
    link().up.push_back(frame);
}

/// Every frame the capsule has sent down since the last call.
pub fn sent() -> Vec<Vec<u8>> {
    core::mem::take(&mut link().down)
}

/// A polled packet: its source, destination and payload.
pub type Polled = ([u8; 4], [u8; 4], Vec<u8>);

/// Poll for a packet of `protocol` through the capsule's OP_POLL_PACKET
/// handler: the status, then the source, destination and payload.
pub fn poll(protocol: u8) -> (u16, Option<Polled>) {
    let mut req = Vec::new();
    req.extend_from_slice(&MAGIC.to_le_bytes());
    req.extend_from_slice(&1u16.to_le_bytes());
    req.extend_from_slice(&OP_POLL_PACKET.to_le_bytes());
    req.extend_from_slice(&[0, 0, 0, 0]);
    req.extend_from_slice(&5u32.to_le_bytes());
    req.extend_from_slice(&1u32.to_le_bytes());
    req.push(protocol);
    let (r, body) = parse(&req).expect("a well formed request");
    let mut tx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    handlers::poll_packet::handle(APP, &r, body, &mut tx);
    let b = nonos_libc::take_replies().pop().expect("the handler replied").bytes;
    let errno = u16::from_le_bytes([b[8], b[9]]);
    let len = u32::from_le_bytes([b[16], b[17], b[18], b[19]]) as usize;
    if errno != 0 || len < 9 {
        return (errno, None);
    }
    let body = &b[20..20 + len];
    let src = [body[0], body[1], body[2], body[3]];
    let dst = [body[4], body[5], body[6], body[7]];
    (errno, Some((src, dst, body[9..].to_vec())))
}
